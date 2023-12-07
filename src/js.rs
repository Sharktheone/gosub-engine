use lazy_static::lazy_static;
use thiserror::Error;

pub use context::*;
pub use runtime::*;
pub use value::*;
pub use value_conversion::*;

use crate::js::v8::V8Engine;
use crate::types::Result;

pub mod v8;
mod context;
mod runtime;
mod value;
mod value_conversion;

#[derive(Error, Debug)]
pub enum JSError {
    #[error("generic error: {0}")]
    Generic(String),

    #[error("conversion error: {0}")]
    Conversion(String),
}


lazy_static!(
    static ref RUNTIME: Runtime<V8Engine<'static>> = runtime::Runtime::new();
);



pub trait JSObject {
    type Value: JSValue;

    fn set_property(&self, name: &str, value: &str) -> Result<()>;

    fn get_property(&self, name: &str) -> Result<Self::Value>;

    fn call_method(&self, name: &str, args: &[&str]) -> Result<Self::Value>;

    fn set_method(&self, name: &str, function: &str) -> Result<()>;
}


pub trait JSArray {
    type Value: JSValue;


    fn get(&self, index: usize) -> Result<Self::Value>;

    fn set(&self, index: usize, value: &str) -> Result<()>;

    fn push(&self, value: &str) -> Result<()>;

    fn pop(&self) -> Result<Self::Value>;

    fn length(&self) -> Result<usize>;

    //TODO: implement other things when needed. Maybe also `Iterator`?
}

pub enum JSType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Object,
    Array,
    Function,
}