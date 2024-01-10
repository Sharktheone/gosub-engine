use lazy_static::lazy_static;
use std::sync::Mutex;
use thiserror::Error;

use crate::web_executor::js::v8::V8Engine;
pub use compile::*;
pub use context::*;
pub use function::*;
pub use runtime::*;
pub use value::*;
pub use value_conversion::*;

use crate::types::Result;

mod compile;
mod context;
mod function;
mod runtime;
pub mod v8;
mod value;
mod value_conversion;

#[derive(Error, Debug)]
pub enum JSError {
    #[error("generic error: {0}")]
    Generic(String),

    #[error("conversion error: {0}")]
    Conversion(String),

    #[error("runtime error: {0}")]
    Runtime(String),

    #[error("compile error: {0}")]
    Compile(String),

    #[error("initialize error: {0}")]
    Initialize(String),

    #[error("execution error: {0}")]
    Execution(String),
}

lazy_static! {
    pub static ref RUNTIME: Mutex<V8Engine<'static>> = Mutex::new(V8Engine::new());
}

pub trait JSObject {
    type Runtime: JSRuntime;

    fn set_property(&self, name: &str, value: &<Self::Runtime as JSRuntime>::Value) -> Result<()>;

    fn get_property(&self, name: &str) -> Result<<Self::Runtime as JSRuntime>::Value>;

    fn call_method(&self, name: &str, args: &[&<Self::Runtime as JSRuntime>::Value]) -> Result<<Self::Runtime as JSRuntime>::Value>;

    fn set_method(&self, name: &str, func: &<Self::Runtime as JSRuntime>::Function) -> Result<()>;
}

pub trait JSArray {
    type Runtime: JSRuntime;

    type Index;

    fn get<T: Into<Self::Index>>(&self, index: T) -> Result<<Self::Runtime as JSRuntime>::Value>;

    fn set<T: Into<Self::Index>>(&self, index: T, value: &<Self::Runtime as JSRuntime>::Value) -> Result<()>;

    fn push(&self, value: <Self::Runtime as JSRuntime>::Value) -> Result<()>;

    fn pop(&self) -> Result<<Self::Runtime as JSRuntime>::Value>;

    fn remove<T: Into<Self::Index>>(&self, index: T) -> Result<()>;

    fn length(&self) -> Result<Self::Index>;

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
    Other(String),
}
