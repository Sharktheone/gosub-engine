use core::fmt::Display;

use gosub_shared::types::Result;

use crate::js::IntoRustValue;
use crate::js::WebRuntime;

//trait for JS functions (interop between JS and Rust)
pub trait WebFunction<RT: WebRuntime> {
    fn new(ctx: RT::Context, func: impl Fn(&mut RT::FunctionCallBack) + 'static) -> Result<Self>
    where
        Self: Sized;

    fn call(&mut self, args: &[RT::Value]) -> Result<RT::Value>;
}

pub trait WebFunctionCallBack<RT: WebRuntime> {
    fn context(&mut self) -> RT::Context;

    fn args(&mut self) -> &RT::Args;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn error(&mut self, error: impl Display);

    fn ret(&mut self, value: RT::Value);
}

pub trait Args<RT: WebRuntime>: Iterator {
    fn get(&self, index: usize, ctx: RT::Context) -> Option<RT::Value>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn as_vec(&self, ctx: RT::Context) -> Vec<RT::Value>;
}

//extra trait for variadic functions to mark them as such
pub trait WebFunctionVariadic<RT: WebRuntime> {
    fn new(ctx: RT::Context, func: impl Fn(&mut RT::FunctionCallBackVariadic) + 'static) -> Result<Self>
    where
        Self: Sized;

    fn call(&mut self, args: &[RT::Value]) -> Result<RT::Value>;
}

pub trait WebFunctionCallBackVariadic<RT: WebRuntime> {
    fn context(&mut self) -> RT::Context;

    fn args(&mut self) -> &RT::VariadicArgsInternal;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn error(&mut self, error: impl Display);

    fn ret(&mut self, value: RT::Value);
}

pub trait VariadicArgsInternal<RT: WebRuntime>: Iterator {
    fn get(&self, index: usize, ctx: RT::Context) -> Option<RT::Value>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn as_vec(&self, ctx: RT::Context) -> Vec<RT::Value>;

    fn variadic(&self, ctx: RT::Context) -> RT::VariadicArgs;

    fn variadic_start(&self, start: usize, ctx: RT::Context) -> RT::VariadicArgs;
}

pub trait VariadicArgs<RT: WebRuntime> {
    fn get(&self, index: usize) -> Option<&RT::Value>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn as_vec(&self) -> &Vec<RT::Value>;

    fn as_vec_as<T>(&self) -> Vec<T>
    where
        RT::Value: IntoRustValue<T>;

    fn get_as<T>(&self, index: usize) -> Option<T>
    where
        RT::Value: IntoRustValue<T>;
}
