use crate::web_executor::js::{JSContext, JSError, JSObject, JSRuntime, JSValue};

struct Function<T: JSFunction>(pub T);

//trait for JS functions (interopt between JS and Rust)
pub trait JSFunction {
    type Runtime: JSRuntime;

    fn call(&mut self, callback: &mut <Self::Runtime as JSRuntime>::CB);
}

pub trait JSFunctionCallBack {
    type Runtime: JSRuntime;

    fn context(&mut self) -> <Self::Runtime as JSRuntime>::Context;

    fn args(&mut self) -> Vec<<Self::Runtime as JSRuntime>::Value>;

    fn ret(&mut self, value: <Self::Runtime as JSRuntime>::Value);
}

pub trait VariadicArgs: Iterator {
    type Runtime: JSRuntime;

    fn get(&self, index: usize) -> Option<<Self::Runtime as JSRuntime>::Value>;

    fn len(&self) -> usize;

    fn as_vec(&self) -> Vec<<Self::Runtime as JSRuntime>::Value>;
}

pub trait Args: Iterator {
    type Runtime: JSRuntime;

    fn get(&self, index: usize) -> Option<<Self::Runtime as JSRuntime>::Value>;

    fn len(&self) -> usize;

    fn as_vec(&self) -> Vec<<Self::Runtime as JSRuntime>::Value>;
}

pub struct VariadicFunction<T: JSFunctionVariadic>(pub T);

//extra trait for variadic functions to mark them as such
pub trait JSFunctionVariadic {
    type Runtime: JSRuntime;

    fn call(&mut self, callback: &mut <Self::Runtime as JSRuntime>::VariadicCB);
}

pub trait JSFunctionCallBackVariadic {
    type Runtime: JSRuntime;

    fn scope(&mut self) -> <Self::Runtime as JSRuntime>::Context;

    fn args(&mut self) -> &<Self::Runtime as JSRuntime>::VariadicArgs;

    fn ret(&mut self, value: <Self::Runtime as JSRuntime>::Value);

    fn error(&mut self, error: JSError);
}
