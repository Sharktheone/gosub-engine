use alloc::rc::Rc;

use v8::{Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

use crate::web_executor::js::function::{JSFunctionCallBack, JSFunctionCallBackVariadic};
use crate::web_executor::js::v8::{V8Context, V8Engine, V8Value};
use crate::web_executor::js::{Args, JSError, JSFunction, JSFunctionVariadic, JSRuntime, JSValue, VariadicArgs};
use crate::types::{Error, Result};

pub struct V8Function<'a> {
    pub(super) ctx: V8Context<'a>,
    pub(super) function: Local<'a, Function>,
}

pub struct V8FunctionVariadic<'a> {
    pub(super) ctx: V8Context<'a>,
    pub(super) function: Local<'a, Function>,
}

impl<'a> V8FunctionVariadic<'a> {
    pub fn new(ctx: V8Context<'a>, function: Local<'a, Function>) -> Self {
        Self { ctx, function }
    }
}

pub struct V8FunctionCallBack<'a, 'args> {
    ctx: V8Context<'a>,
    args: V8Args<'a, 'args>,
    ret: Result<V8Value<'a>>,
}

pub struct V8Args<'a, 'args> {
    ctx: V8Context<'a>,
    next: usize,
    args: Vec<Local<'args, v8::Value>>,
}

impl V8Args<'_, '_> {
    fn v8(&self) -> &[Local<v8::Value>] {
        &self.args
    }
}

impl<'a> Iterator for V8Args<'a, 'a> {
    type Item = V8Value<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < self.args.len() {
            let value = *self.args.get(self.next)?;
            self.next += 1;
            Some(V8Value {
                context: Rc::clone(&self.ctx),
                value,
            })
        } else {
            None
        }
    }
}

impl<'a> Args for V8Args<'a, 'a> {
    type Runtime = V8Engine<'a>;

    fn get(&self, index: usize) -> Option<<Self::Runtime as JSRuntime>::Value> {
        if index < self.args.len() {
            Some(V8Value {
                context: Rc::clone(&self.ctx),
                value: *self.args.get(index)?,
            })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.args.len()
    }

    fn as_vec(&self) -> Vec<<Self::Runtime as JSRuntime>::Value> {
        let mut a = Vec::with_capacity(self.args.len());
        for i in 0..self.args.len() {
            let Some(value) = self.args.get(i) else {
                continue;
            };

            a.push(V8Value {
                context: Rc::clone(&self.ctx),
                value: *value,
            });
        }

        a
    }
}

impl<'a> JSFunctionCallBack for V8FunctionCallBack<'a, 'a> {
    type Runtime = V8Engine<'a>;

    fn context(&mut self) -> <Self::Runtime as JSRuntime>::Context {
        Rc::clone(&self.ctx)
    }

    fn args(&mut self) -> Vec<<Self::Runtime as JSRuntime>::Value> {
        self.args.as_vec()
    }

    fn ret(&mut self, value: <Self::Runtime as JSRuntime>::Value) {
        self.ret = Ok(value);
    }
}

impl<'a> V8Function<'a> {
    pub fn new(ctx: V8Context<'a>, f: impl Fn(&mut V8FunctionCallBack)) -> Result<V8Function> {
        let ctx = Rc::clone(&ctx);

        let function = Function::new(
            ctx.borrow_mut().scope(),
            |scope: &mut HandleScope, args: FunctionCallbackArguments, mut ret: ReturnValue| {
                let mut a = Vec::with_capacity(args.length() as usize);

                for i in 0..args.length() {
                    a.push(args.get(i));
                }

                let args = V8Args {
                    ctx: Rc::clone(&ctx),
                    next: 0,
                    args: a,
                };

                let mut cb = V8FunctionCallBack {
                    ctx: Rc::clone(&ctx),
                    args,
                    ret: Err(Error::JS(JSError::Execution(
                        "function was not called".to_owned(),
                    ))),
                };

                f(&mut cb);

                match cb.ret {
                    Ok(value) => {
                        ret.set(value.value);
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
            },
        );

        if let Some(function) = function {
            Ok(Self { ctx, function })
        } else {
            Err(Error::JS(JSError::Compile(
                "failed to create function".to_owned(),
            )))
        }
    }
}

impl<'a> JSFunction for V8Function<'a> {
    type Runtime = V8Engine<'a>;

    fn call(&mut self, cb: &mut V8FunctionCallBack) {
        let ret = self.function.call(
            cb.ctx.borrow_mut().scope(),
            Local::from(v8::undefined(cb.ctx.borrow_mut().scope())),
            cb.args.v8(),
        );

        if let Some(value) = ret {
            cb.ret = Ok(V8Value::from_value(Rc::clone(&cb.ctx), value));
        } else {
            cb.ret = Err(Error::JS(JSError::Execution(
                "failed to call a function".to_owned(),
            )));
        };
    }
}

pub struct V8VariadicArgs<'a> {
    ctx: V8Context<'a>,
    next: i32,
    args: FunctionCallbackArguments<'a>,
}

impl V8VariadicArgs<'_> {
    fn v8(&self) -> Vec<Local<v8::Value>> {
        let mut a = Vec::with_capacity(self.args.length() as usize);
        for i in 0..self.args.length() {
            a.push(self.args.get(i));
        }

        a
    }
}

impl<'a> Iterator for V8VariadicArgs<'a> {
    type Item = V8Value<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.next < 0 {
            self.next = 0;
        }

        if self.next < self.args.length() {
            let value = self.args.get(self.next);
            self.next += 1;
            Some(V8Value {
                context: Rc::clone(&self.ctx),
                value,
            })
        } else {
            None
        }
    }
}

impl<'a> VariadicArgs for V8VariadicArgs<'a> {
    type Runtime = V8Engine<'a>;

    fn get(&self, index: usize) -> Option<<Self::Runtime as JSRuntime>::Value> {
        if index < self.args.length() as usize {
            Some(V8Value {
                context: Rc::clone(&self.ctx),
                value: self.args.get(index as i32),
            })
        } else {
            None
        }
    }

    fn len(&self) -> usize {
        self.args.length() as usize
    }

    fn as_vec(&self) -> Vec<<Self::Runtime as JSRuntime>::Value> {
        let mut a = Vec::with_capacity(self.args.length() as usize);
        for i in 0..self.args.length() {
            a.push(V8Value {
                context: Rc::clone(&self.ctx),
                value: self.args.get(i),
            });
        }

        a
    }
}

pub struct V8FunctionCallBackVariadic<'a> {
    ctx: V8Context<'a>,
    args: V8VariadicArgs<'a>,
    ret: Result<V8Value<'a>>,
}

impl<'a> JSFunctionCallBackVariadic for V8FunctionCallBackVariadic<'a> {

    type Runtime = V8Engine<'a>;

    fn scope(&mut self) -> <Self::Runtime as JSRuntime>::Context {
        Rc::clone(&self.ctx)
    }

    fn args(&mut self) -> &<Self::Runtime as JSRuntime>::VariadicArgs {
        &self.args
    }

    fn ret(&mut self, value: <Self::Runtime as JSRuntime>::Value) {
        self.ret = Ok(value);
    }

    fn error(&mut self, error: JSError) {
        self.ret = Err(Error::JS(error));
    }
}

impl<'a> JSFunctionVariadic for V8FunctionVariadic<'a> {
    type Runtime = V8Engine<'a>;

    fn call(&mut self, cb: &mut <Self::Runtime as JSRuntime>::VariadicCB) {
        let ret = self.function.call(
            cb.ctx.borrow_mut().scope(),
            Local::from(v8::undefined(cb.ctx.borrow_mut().scope())),
            &cb.args.v8(),
        );

        if let Some(value) = ret {
            cb.ret = Ok(V8Value::from_value(Rc::clone(&cb.ctx), value));
        } else {
            cb.ret = Err(Error::JS(JSError::Execution(
                "failed to call a function".to_owned(),
            )));
        };
    }
}
