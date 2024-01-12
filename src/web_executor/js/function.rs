use crate::web_executor::js::{JSContext, JSError, JSObject, JSRuntime, JSValue};

struct Function<T: JSFunction>(pub T);

//trait for JS functions (interop between JS and Rust)
pub trait JSFunction {
    type CB: JSFunctionCallBack;

    fn call(&mut self, callback: &mut Self::CB);
}

pub trait JSFunctionCallBack {
    type Args: Args;
    type Context: JSContext;
    type Value: JSValue;

    fn context(&mut self) -> Self::Context;

    fn args(&mut self) -> &Self::Args;

    fn len(&self) -> usize;

    fn ret(&mut self, value: Self::Value);
}

pub trait VariadicArgs: Iterator {
    type Value: JSValue;

    fn get(&self, index: usize) -> Option<Self::Value>;

    fn len(&self) -> usize;

    fn as_vec(&self) -> Vec<Self::Value>;
}

pub trait Args: Iterator {
    type Context: JSContext;
    type Value: JSValue;

    fn get(&self, index: usize, ctx: Self::Context) -> Option<Self::Value>;

    fn len(&self) -> usize;

    fn as_vec(&self, ctx: Self::Context) -> Vec<Self::Value>;
}

pub struct VariadicFunction<T: JSFunctionVariadic>(pub T);

//extra trait for variadic functions to mark them as such
pub trait JSFunctionVariadic {
    type VariadicCB: JSFunctionCallBackVariadic;

    fn call(&mut self, callback: &mut Self::VariadicCB);
}

pub trait JSFunctionCallBackVariadic {
    type Context: JSContext;
    type Value: JSValue;
    type VariadicArgs: VariadicArgs;

    fn scope(&mut self) -> Self::Context;

    fn args(&mut self) -> &Self::VariadicArgs;

    fn ret(&mut self, value: Self::Value);

    fn error(&mut self, error: JSError);
}
