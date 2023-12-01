use thiserror::Error;
use crate::js::v8::V8Context;

use crate::types::Result;

mod v8;

#[derive(Error, Debug)]
pub enum Error {
    #[error("generic error: {0}")]
    Generic(String),
}

pub trait JSRuntime {
    type Context;

    fn new() -> Result<Self> where Self: Sized;

    fn new_context(&self) -> Result<Self::Context>;
}

pub trait JSContext {
    fn run(&self, code: &str) -> Result<()>;

    fn compile(&self, code: &str) -> Result<()>;

    fn run_compiled(&self) -> Result<()>;

    // fn compile_stream(&self, code: &str) -> Result<()>;

    fn add_global_object(&self, name: &str, object: &str) -> Result<()>;

    fn add_function_to_object(&self, object: &str, name: &str, function: &str) -> Result<()>;
}