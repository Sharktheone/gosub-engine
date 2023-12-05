use lazy_static::lazy_static;
use thiserror::Error;
use crate::js::context::Context;

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

    fn add_global_object(&self, name: &str, object: &str) -> Result<()>;

    fn add_function_to_object(&self, object: &str, name: &str, function: &str) -> Result<()>;
}