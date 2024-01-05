use crate::js::{JSContext, JSError, JSObject, JSValue};

struct Function<T: JSFunction>(pub T);

//trait for JS functions (interopt between JS and Rust)
pub trait JSFunction {
    type Context: JSContext;
    type CB: JSFunctionCallBack;

    fn call(&mut self, callback: &mut Self::CB);
}

pub trait JSFunctionCallBack {
    type Context: JSContext;

    type Value: JSValue;

    fn context(&mut self) -> Self::Context;

    fn args(&mut self) -> Vec<Self::Value>;

    fn ret(&mut self, value: Self::Value);
}

pub trait VariadicArgs: Iterator {
    type Value: JSValue;

    fn get(&self, index: usize) -> Option<Self::Value>;

    fn len(&self) -> usize;

    fn as_vec(&self) -> Vec<Self::Value>;
}

pub trait Args: Iterator {
    type Value: JSValue;

    fn get(&self, index: usize) -> Option<Self::Value>;

    fn len(&self) -> usize;

    fn as_vec(&self) -> Vec<Self::Value>;
}

pub struct VariadicFunction<T: JSFunctionVariadic>(pub T);

//extra trait for variadic functions to mark them as such
pub trait JSFunctionVariadic {
    type Context: JSContext;

    type CB: JSFunctionCallBackVariadic;

    fn call(&mut self, callback: &mut Self::CB);
}

pub trait JSFunctionCallBackVariadic {
    type Context: JSContext;

    type Value: JSValue;

    type Args: VariadicArgs;

    fn scope(&mut self) -> Self::Context;

    fn args(&mut self) -> &Self::Args;

    fn ret(&mut self, value: Self::Value);

    fn error(&mut self, error: JSError);
}
