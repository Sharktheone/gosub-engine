use lazy_static::lazy_static;
use thiserror::Error;

use crate::types::Result;

pub mod v8;
mod context;
mod runtime;

use crate::js::context::Context;
use crate::js::v8::V8Engine;
// not sure if this is a good idea... I mean, we probably want to decide the JSRuntime at compile time and not at runtime of the engine.
// So maybe we should rethink the whole trait situation. Maybe we actually don't need the traits at all, and just use the concrete types with this solution.
// Maybe we also could just use traits, but probably we don't really benefit from them, because we don't need to switch the JSRuntime at runtime of the engine.




#[derive(Error, Debug)]
pub enum Error {
    #[error("generic error: {0}")]
    Generic(String),
}


lazy_static!(
    static ref RUNTIME: Runtime<V8Engine> = runtime::Runtime::new();
);


pub trait JSRuntime {
    type Context: JSContext;

    fn new_context(&self) -> Result<Context<Self::Context>>;
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

    type Value: JSValue;

    fn set_property(&self, name: &str, value: &str) -> Result<()>;

    fn get_property(&self, name: &str) -> Result<Self::Value>;

    fn call_method(&self, name: &str, args: &[&str]) -> Result<Self::Value>;

    fn set_method(&self, name: &str, function: &str) -> Result<()>;
}

pub trait JSValue {

    type Object: JSObject;
    type Array: JSArray;

    fn as_string(&self) -> Result<String>;

    fn as_number(&self) -> Result<f64>;

    fn as_bool(&self) -> Result<bool>;

    fn as_object(&self) -> Result<Self::Object>;

    fn as_array(&self) -> Result<Self::Array>;

    fn is_string(&self) -> bool;

    fn is_number(&self) -> bool;

    fn is_bool(&self) -> bool;

    fn is_object(&self) -> bool;

    fn is_array(&self) -> bool;

    fn is_null(&self) -> bool;

    fn is_undefined(&self) -> bool;

    fn is_function(&self) -> bool;

    fn type_of(&self) -> JSType;

    fn new_object() -> Result<Self::Object>;

    fn new_array<T: ValueTranslation>(value: &[T]) -> Result<Self::Array>;

    fn new_string(value: &str) -> Result<Self>;

    fn new_number(value: f64) -> Result<Self>;

    fn new_bool(value: bool) -> Result<Self>;

    fn new_null() -> Result<Self>;

    fn new_undefined() -> Result<Self>;

    fn new_function(func: &fn(/*Input arguments, return type, some kind of context (HandleScope for V8)*/)) -> Result<Self>; //Is a function also a value? I think so, but I'm not sure.
}


pub trait ValueTranslation {
    type Value: JSValue;

    fn to_value(&self) -> Self::Value;
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

enum JSType {
    Undefined,
    Null,
    Boolean,
    Number,
    String,
    Object,
    Array,
    Function,
}