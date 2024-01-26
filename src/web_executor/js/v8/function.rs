use alloc::rc::Rc;

use v8::{CallbackScope, External, Function, FunctionBuilder, FunctionCallbackArguments, FunctionCallbackInfo, HandleScope, Local, ReturnValue};

use crate::types::{Error, Result};
use crate::web_executor::js::{Args, JSError, JSFunction, JSFunctionVariadic, JSRuntime, JSValue, VariadicArgs, VariadicArgsInternal};
use crate::web_executor::js::function::{JSFunctionCallBack, JSFunctionCallBackVariadic};
use crate::web_executor::js::v8::{V8Context, V8Value};

pub struct V8Function<'a> {
    pub(super) ctx: V8Context<'a>,
    pub(super) function: Local<'a, Function>,
}

pub struct V8FunctionCallBack<'a> {
    ctx: V8Context<'a>,
    args: V8Args<'a>,
    ret: Result<Local<'a, v8::Value>>,
}

pub struct V8Args<'a> {
    next: usize,
    args: Vec<Local<'a, v8::Value>>,
}

impl V8Args<'_> {
    fn v8(&self) -> &[Local<v8::Value>] {
        &self.args
    }
}

impl<'a> Iterator for V8Args<'a> {
    type Item = Local<'a, v8::Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.args.len() {
            let value = *self.args.get(self.next)?;
            self.next += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<'a> Args for V8Args<'a> {
    type Context = V8Context<'a>;
    type Value = V8Value<'a>;

    fn get(&self, index: usize, ctx: Self::Context) -> Option<Self::Value> {
        if index < self.args.len() {
            Some(V8Value {
                context: Rc::clone(&ctx),
                value: *self.args.get(index)?,
            })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.args.len()
    }

    fn as_vec(&self, ctx: Self::Context) -> Vec<Self::Value> {
        let mut a = Vec::with_capacity(self.args.len());
        for i in 0..self.args.len() {
            let Some(value) = self.args.get(i) else {
                continue;
            };

            a.push(V8Value {
                context: Rc::clone(&ctx),
                value: *value,
            });
        }

        a
    }
}

impl<'a> JSFunctionCallBack for V8FunctionCallBack<'a> {
    type Args = V8Args<'a>;
    type Context = V8Context<'a>;
    type Value = V8Value<'a>;

    fn context(&mut self) -> Self::Context {
        Rc::clone(&self.ctx)
    }

    fn args(&mut self) -> &Self::Args {
        &self.args
    }

    fn len(&self) -> usize {
        self.args.len()
    }

    fn ret(&mut self, value: Self::Value) {
        self.ret = Ok(value.value);
    }
}

impl<'a> V8Function<'a> {
    pub(crate) fn callback(
        ctx: &V8Context<'a>,
        scope: &mut HandleScope,
        args: FunctionCallbackArguments<'a>,
        mut ret: ReturnValue,
        f: impl Fn(&mut V8FunctionCallBack<'a>),
    ) {
        let mut a = Vec::with_capacity(args.length() as usize);

        for i in 0..args.length() {
            a.push(args.get(i));
        }

        let args = V8Args { next: 0, args: a };

        let mut cb = V8FunctionCallBack {
            ctx: Rc::clone(ctx),
            args,
            ret: Err(Error::JS(JSError::Execution(
                "function was not called".to_owned(),
            ))),
        };

        let t = cb.args().len();

        f(&mut cb);

        match cb.ret {
            Ok(value) => {
                ret.set(value);
            }
            Err(e) => {
                let excep = if let Some(exception) =
                    v8::String::new(ctx.borrow_mut().scope(), &e.to_string())
                {
                    exception.into()
                } else {
                    eprintln!("failed to create exception string\nexception was: {e}"); //TODO: replace with our own logger
                    v8::undefined(ctx.borrow_mut().scope()).into()
                };

                ret.set(ctx.borrow_mut().scope().throw_exception(excep));
            }
        }
    }
}

extern "C" fn callback<F>(info: *const FunctionCallbackInfo)
    where
        F: FnOnce(&mut HandleScope, FunctionCallbackArguments, ReturnValue),
{
    let info = unsafe { &*info };
    let scope = &mut unsafe { CallbackScope::new(info) };
    let args = FunctionCallbackArguments::from_function_callback_info(info);
    let rv = ReturnValue::from_function_callback_info(info);
    let external = match <Local<External>>::try_from(args.data()) {
        Ok(external) => external,
        Err(e) => {
            let Some(e) = v8::String::new(scope, &e.to_string()) else {
                eprintln!("failed to create exception string\nexception was: {e}"); //TODO: replace with our own logger
                return;
            };
            scope.throw_exception(Local::from(e));
            return;
        }
    };

    let closure = unsafe { &mut *(external.value() as *mut F) };
    closure(scope, args, rv);
}

struct CallbackWrapper<'a> {
    ctx: V8Context<'a>,
    f: Box<dyn Fn(&mut V8FunctionCallBack<'a>)>,
}

impl<'a> CallbackWrapper<'a> {
    fn new(ctx: V8Context<'a>, f: impl Fn(&mut V8FunctionCallBack<'a>) + 'static)) -> Self {
        Self {
            ctx,
            f: Box::new(f),
        }
    }
}

fn get_callback<F>(_: &F) -> extern "C" fn(info: *const FunctionCallbackInfo)
    where
        F: FnOnce(&mut HandleScope, FunctionCallbackArguments, ReturnValue),
{
    callback::<F>
}

impl<'a> JSFunction for V8Function<'a> {
    type CB = V8FunctionCallBack<'a>;
    type Context = V8Context<'a>;
    type Value = V8Value<'a>;

    fn new(ctx: Self::Context, f: impl Fn(&mut Self::CB)) -> Result<Self> {
        let ctx = Rc::clone(&ctx);


        let mut closure = {
            let ctx = Rc::clone(&ctx);
            |scope: &mut HandleScope<'a>, args: FunctionCallbackArguments<'a>, mut rv: ReturnValue| {
                let ctx = &ctx;
                V8Function::callback(ctx, scope, args, rv, &f);
            }
        };

        let builder: FunctionBuilder<Function> = FunctionBuilder::new_raw(get_callback(&closure));

        let scope = ctx.borrow_mut().scope();

        let closure = External::new(
            scope,
            &mut closure as *mut _ as *mut std::ffi::c_void,
        );

        let function = builder.data(Local::from(closure)).build(scope);

        if let Some(function) = function {
            Ok(Self { ctx, function })
        } else {
            Err(Error::JS(JSError::Compile(
                "failed to create function".to_owned(),
            )))
        }
    }

    fn call(&mut self, cb: &mut V8FunctionCallBack) {
        let ret = self.function.call(
            cb.ctx.borrow_mut().scope(),
            Local::from(v8::undefined(cb.ctx.borrow_mut().scope())),
            cb.args.v8(),
        );

        if let Some(value) = ret {
            cb.ret = Ok(value);
        } else {
            cb.ret = Err(Error::JS(JSError::Execution(
                "failed to call a function".to_owned(),
            )));
        };
    }
}

//TODO: maybe move both implementations into a macro, so we have less code duplication

pub struct V8FunctionVariadic<'a> {
    pub(super) ctx: V8Context<'a>,
    pub(super) function: Local<'a, Function>,
}

pub struct V8FunctionCallBackVariadic<'a> {
    ctx: V8Context<'a>,
    args: V8VariadicArgsInternal<'a>,
    ret: Result<Local<'a, v8::Value>>,
}

pub struct V8VariadicArgsInternal<'a> {
    next: usize,
    args: Vec<Local<'a, v8::Value>>,
}

impl V8VariadicArgsInternal<'_> {
    fn v8(&self) -> &[Local<v8::Value>] {
        &self.args
    }
}

impl<'a> Iterator for V8VariadicArgsInternal<'a> {
    type Item = Local<'a, v8::Value>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.args.len() {
            let value = *self.args.get(self.next)?;
            self.next += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<'a> VariadicArgsInternal for V8VariadicArgsInternal<'a> {
    type Context = V8Context<'a>;
    type Value = V8Value<'a>;

    type Args = V8VariadicArgs<'a>;

    fn get(&self, index: usize, ctx: Self::Context) -> Option<Self::Value> {
        if index < self.args.len() {
            Some(V8Value {
                context: Rc::clone(&ctx),
                value: *self.args.get(index)?,
            })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.args.len()
    }

    fn as_vec(&self, ctx: Self::Context) -> Vec<Self::Value> {
        let mut a = Vec::with_capacity(self.args.len());
        for i in 0..self.args.len() {
            let Some(value) = self.args.get(i) else {
                continue;
            };

            a.push(V8Value {
                context: Rc::clone(&ctx),
                value: *value,
            });
        }

        a
    }

    fn variadic(&self, ctx: Self::Context) -> Self::Args {
        V8VariadicArgs {
            next: 0,
            args: self.as_vec(ctx),
        }
    }
}

pub struct V8VariadicArgs<'a> {
    next: usize,
    args: Vec<V8Value<'a>>,
}

impl<'a> VariadicArgs for V8VariadicArgs<'a> {
    type Value = V8Value<'a>;

    fn get(&self, index: usize) -> Option<&Self::Value> {
        self.args.get(index)
    }

    fn len(&self) -> usize {
        self.args.len()
    }

    fn as_vec(&self) -> &Vec<Self::Value> {
        &self.args
    }
}

impl<'a> JSFunctionCallBackVariadic for V8FunctionCallBackVariadic<'a> {
    type Args = V8VariadicArgsInternal<'a>;
    type Context = V8Context<'a>;
    type Value = V8Value<'a>;

    fn context(&mut self) -> Self::Context {
        Rc::clone(&self.ctx)
    }

    fn args(&mut self) -> &Self::Args {
        &self.args
    }

    fn len(&self) -> usize {
        self.args.len()
    }

    fn ret(&mut self, value: Self::Value) {
        self.ret = Ok(value.value);
    }
}

impl<'a> V8FunctionVariadic<'a> {
    fn callback(
        ctx: &V8Context<'a>,
        scope: &mut HandleScope,
        args: FunctionCallbackArguments<'a>,
        mut ret: ReturnValue,
        f: impl Fn(&mut V8FunctionCallBackVariadic<'a>),
    ) {
        let mut a = Vec::with_capacity(args.length() as usize);

        for i in 0..args.length() {
            a.push(args.get(i));
        }

        let args = V8VariadicArgsInternal { next: 0, args: a };

        let mut cb = V8FunctionCallBackVariadic {
            ctx: Rc::clone(ctx),
            args,
            ret: Err(Error::JS(JSError::Execution(
                "function was not called".to_owned(),
            ))),
        };

        let t = cb.args().len();

        f(&mut cb);

        match cb.ret {
            Ok(value) => {
                ret.set(value);
            }
            Err(e) => {
                let excep = if let Some(exception) =
                    v8::String::new(ctx.borrow_mut().scope(), &e.to_string())
                {
                    exception.into()
                } else {
                    eprintln!("failed to create exception string\nexception was: {e}"); //TODO: replace with our own logger
                    v8::undefined(ctx.borrow_mut().scope()).into()
                };

                ret.set(ctx.borrow_mut().scope().throw_exception(excep));
            }
        }
    }
}

impl<'a> JSFunctionVariadic for V8FunctionVariadic<'a> {
    type CB = V8FunctionCallBackVariadic<'a>;
    type Context = V8Context<'a>;
    type Value = V8Value<'a>;

    fn new(ctx: Self::Context, f: impl Fn(&mut Self::CB)) -> Result<Self> {
        let ctx = Rc::clone(&ctx);
        let scope = ctx.borrow_mut().scope();


        let mut closure = {
            let ctx = Rc::clone(&ctx);
            move |scope: &mut HandleScope, args: FunctionCallbackArguments<'a>, mut rv: ReturnValue, | {
            V8FunctionVariadic::callback(&ctx, scope, args, rv, &f);
        }
        };

        let builder: FunctionBuilder<Function> = FunctionBuilder::new_raw(get_callback(&closure));

        let closure = External::new(
            scope,
            &mut closure as *mut _ as *mut std::ffi::c_void,
        );

        let function = builder.data(Local::from(closure)).build(scope);

        if let Some(function) = function {
            Ok(Self { ctx, function })
        } else {
            Err(Error::JS(JSError::Compile(
                "failed to create function".to_owned(),
            )))
        }
    }

    fn call(&mut self, cb: &mut Self::CB) {
        let ret = self.function.call(
            cb.ctx.borrow_mut().scope(),
            Local::from(v8::undefined(cb.ctx.borrow_mut().scope())),
            cb.args.v8(),
        );

        if let Some(value) = ret {
            cb.ret = Ok(value);
        } else {
            cb.ret = Err(Error::JS(JSError::Execution(
                "failed to call a function".to_owned(),
            )));
        };
    }
}
