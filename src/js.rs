use thiserror::Error;
use crate::js::v8::V8Context;

use crate::types::Result;

mod v8;
mod context;
mod runtime;

#[derive(Error, Debug)]
pub enum Error {
    #[error("generic error: {0}")]
    Generic(String),
}

lazy_static!(
    static ref RUNTIME:  = v8::V8Engine::new();
);


pub trait JSRuntime {
    type Context: JSContext;

    fn new_context(&self) -> Result<Context<Self::Context>>;
}

pub trait JSContext {
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