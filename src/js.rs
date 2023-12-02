use thiserror::Error;

use crate::types::Result;

#[cfg(feature = "v8")]
pub mod v8;


#[cfg(feature = "v8")]
pub use v8::runtime_types::*;
// not sure if this is a good idea... I mean, we probably want to decide the JSRuntime at compile time and not at runtime of the engine.
// So maybe we should rethink the whole trait situation. Maybe we actually don't need the traits at all, and just use the concrete types with this solution.
// Maybe we also could just use traits, but probably we don't really benefit from them, because we don't need to switch the JSRuntime at runtime of the engine.




#[derive(Error, Debug)]
pub enum Error {
    #[error("generic error: {0}")]
    Generic(String),
}

pub trait JSRuntime {
    type Context;
    type Object;
    type Value;
    type Array;

    fn new() -> Result<Self> where Self: Sized;

    fn new_context(&self) -> Result<Self::Context>;
}

pub trait JSContext {

    type Object: JSObject;

    fn run(&self, code: &str) -> Result<()>;

    fn compile(&self, code: &str) -> Result<()>;

    fn run_compiled(&self) -> Result<()>;

    // fn compile_stream(&self, code: &str) -> Result<()>;

    fn add_global_object(&self, name: &str) -> Result<Self::Object>;
}


pub trait JSObject {
    fn set_property(&self, name: &str, value: &str) -> Result<()>;

    fn get_property(&self, name: &str) -> Result<String>;

    fn call_method(&self, name: &str, args: &[&str]) -> Result<String>;

    fn set_method(&self, name: &str, function: &str) -> Result<()>;
}

pub trait JSValue {
    fn as_string(&self) -> Result<String>;

    fn as_number(&self) -> Result<f64>;

    fn as_bool(&self) -> Result<bool>;

    fn as_object(&self) -> Result<Box<dyn JSObject>>;

    fn as_array(&self) -> Result<Box<dyn JSArray>>;
}

pub trait JSArray {
    fn get(&self, index: usize) -> Result<Box<dyn JSValue>>;

    fn set(&self, index: usize, value: &str) -> Result<()>;

    fn push(&self, value: &str) -> Result<()>;

    fn pop(&self) -> Result<Box<dyn JSValue>>;

    fn length(&self) -> Result<usize>;
}