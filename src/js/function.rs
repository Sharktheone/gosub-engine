use crate::js::{JSContext, JSObject, JSValue};
use crate::types::Result;


struct Function<T: JSFunction>(pub T);

pub(super) trait JSFunction {
    type Context: JSContext;

    type Value: JSValue;

    type Object: JSObject;

    fn new(ctx: Self::Context, f: impl FnMut(Self::Context, Self::Object, &[Self::Value]) -> Result<Self::Value>) -> Result<Self>;

    fn call(&mut self, ctx: Self::Context, this: Self::Object, args: &[Self::Value]) -> Result<Self::Value>;
}

pub(super) trait JSFunctionCallBack {
    type Context: JSContext;

    type Value: JSValue;

    fn scope(&mut self) -> Self::Context;

    fn args(&mut self) -> &[Self::Value];

    fn ret(&mut self, value: Self::Value);
}


pub(super) struct VariadicArgs<T: JSValue> {
    args: Vec<T>,
}


impl<T: JSValue> VariadicArgs<T> {
    pub fn new(args: Vec<T>) -> Self {
        Self { args }
    }
}

pub(super) struct VariadicFunction<T: JSFunctionVariadic>(pub T);

pub(super) trait JSFunctionVariadic {
    type Context: JSContext;

    type Value: JSValue;

    type Object: JSObject;


    fn new(ctx: Self::Context, f: impl FnMut(Self::Context, Self::Object, VariadicArgs<Self::Value>) -> Result<Self::Value>) -> Result<Self>;

    fn call(&mut self, ctx: Self::Context, this: Self::Object, args: VariadicArgs<Self::Value>) -> Result<Self::Value>;
}

pub(super) trait JSFunctionCallBackVariadic {
    type Context: JSContext;

    type Value: JSValue;

    fn scope(&mut self) -> Self::Context;

    fn args(&mut self) -> VariadicArgs<Self::Value>;

    fn ret(&mut self, value: Self::Value);
}