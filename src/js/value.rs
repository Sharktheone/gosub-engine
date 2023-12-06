use crate::js::{JSArray, JSObject, JSType, ValueTranslation};
use crate::types::Result;

pub trait JSValue
    where Self: Sized
{
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


//TODO: implement this for different rust types