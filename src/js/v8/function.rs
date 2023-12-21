use alloc::rc::Rc;

use v8::{Function, FunctionCallbackArguments, HandleScope, Local, ReturnValue};

use crate::js::function::{JSFunction, JSFunctionVariadic, VariadicArgs};
use crate::js::JSError;
use crate::js::v8::{Ctx, FromContext, V8Object, V8Value};
use crate::types::{Error, Result};

struct V8Function<'a> {
    function: Local<'a, Function>,
}

struct V8FunctionVariadic<'a> {
    function: Local<'a, Function>,
}

impl<'a> V8Function<'a> {
    pub fn new(function: Local<'a, Function>) -> Self {
        Self { function }
    }
}

impl<'a> V8FunctionVariadic<'a> {
    pub fn new(function: Local<'a, Function>) -> Self {
        Self { function }
    }
}


impl<'a> JSFunction for V8Function<'a> {
    type Context = Ctx<'a>;

    type Value = V8Value<'a>;

    type Object = V8Object<'a>;

    fn new(ctx: Self::Context, f: impl FnMut(Self::Context, Self::Object, &[Self::Value]) -> Result<Self::Value>) -> Result<Self> {
        let mut f = f;

        let function = Function::new(
            ctx.borrow_mut().scope(),
            move |scope: &mut HandleScope, args: FunctionCallbackArguments, mut ret: ReturnValue| {
                let mut a = Vec::with_capacity(args.length() as usize);

                for i in 0..args.length() {
                    a.push(V8Value::from_value(Rc::clone(&ctx), args.get(i)));
                }

                let this = V8Object::from_ctx(Rc::clone(&ctx), args.this());

                let r = f(Rc::clone(&ctx), this, a.as_slice());

                if let Ok(value) = r {
                    ret.set(value.value);
                } else {
                    ret.set(scope.throw_exception(args.get(0)));
                }
            },
        );

        if let Some(function) = function {
            Ok(
                Self {
                    function,
                }
            )
        } else {
            Err(Error::JS(JSError::Compile(
                "failed to create a function".to_owned(),
            )))
        }
    }

    fn call(&mut self, ctx: Self::Context, this: Self::Object, args: &[Self::Value]) -> Result<Self::Value> {
        let args: Vec<Local<v8::Value>> = args.iter().map(|v| v.value).collect();

        let ret = self.function.call(ctx.borrow_mut().scope(), Local::from(this.value), args.as_slice());

        if let Some(value) = ret {
            Ok(V8Value::from_value(ctx, value))
        } else {
            Err(Error::JS(JSError::Execution(
                "failed to call a function".to_owned(),
            )))
        }
    }
}

impl<'a> JSFunctionVariadic for V8FunctionVariadic<'a> {
    type Context = Ctx<'a>;

    type Value = V8Value<'a>;

    type Object = V8Object<'a>;

    fn new(ctx: Self::Context, f: impl FnMut(Self::Context, Self::Object, VariadicArgs<Self::Value>) -> Result<Self::Value>) -> Result<Self> {
        let mut f = f;

        let function = Function::new(
            ctx.borrow_mut().scope(),
            move |scope: &mut HandleScope, args: FunctionCallbackArguments, mut ret: ReturnValue| {
                let mut a = Vec::with_capacity(args.length() as usize);

                for i in 0..args.length() {
                    a.push(V8Value::from_value(Rc::clone(&ctx), args.get(i)));
                }

                let this = V8Object::from_ctx(Rc::clone(&ctx), args.this());

                let r = f(Rc::clone(&ctx), this, VariadicArgs::new(a));

                if let Ok(value) = r {
                    ret.set(value.value);
                } else {
                    ret.set(scope.throw_exception(args.get(0)));
                }
            },
        );

        if let Some(function) = function {
            Ok(
                Self {
                    function,
                }
            )
        } else {
            Err(Error::JS(JSError::Compile(
                "failed to create a function".to_owned(),
            )))
        }
    }

    fn call(&mut self, ctx: Self::Context, this: Self::Object, args: VariadicArgs<Self::Value>) -> Result<Self::Value> {
        let args: Vec<Local<v8::Value>> = args.iter().map(|v| v.value).collect();


        let ret = self.function.call(ctx.borrow_mut().scope(), ctx.borrow_mut().global(), args.as_slice());

        if let Some(value) = ret {
            Ok(V8Value::from_value(ctx, value))
        } else {
            Err(Error::JS(JSError::Execution(
                "failed to call a function".to_owned(),
            )))
        }
    }
}