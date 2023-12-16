use crate::js::v8::{V8Array, V8Object};
use crate::js::{JSError, JSType, JSValue, ValueConversion};
use crate::types::Error;
use v8::{HandleScope, Local, Value};

pub struct V8Value<'a>(Local<'a, Value>);

impl<'a> JSValue for V8Value<'a> {
    type Object = V8Object<'a>;
    type Array = V8Array<'a>;

    fn as_string(&self) -> crate::types::Result<String> {
        todo!()
    }

    fn as_number(&self) -> crate::types::Result<f64> {
        let mut scope: HandleScope = todo!();

        if let Some(value) = self.0.number_value(&mut scope) {
            Ok(value)
        } else {
            Err(Error::JS(JSError::Conversion(
                "could not convert to number".to_owned(),
            )))
        }
    }

    fn as_bool(&self) -> crate::types::Result<bool> {
        todo!()
    }

    fn as_object(&self) -> crate::types::Result<Self::Object> {
        todo!()
    }

    fn as_array(&self) -> crate::types::Result<Self::Array> {
        todo!()
    }

    fn is_string(&self) -> bool {
        todo!()
    }

    fn is_number(&self) -> bool {
        todo!()
    }

    fn is_bool(&self) -> bool {
        todo!()
    }

    fn is_object(&self) -> bool {
        todo!()
    }

    fn is_array(&self) -> bool {
        todo!()
    }

    fn is_null(&self) -> bool {
        todo!()
    }

    fn is_undefined(&self) -> bool {
        todo!()
    }

    fn is_function(&self) -> bool {
        todo!()
    }

    fn type_of(&self) -> JSType {
        todo!()
    }

    fn new_object() -> crate::types::Result<Self::Object> {
        todo!()
    }

    fn new_array<T: ValueConversion<Self>>(value: &[T]) -> crate::types::Result<Self::Array> {
        todo!()
    }

    fn new_string(value: &str) -> crate::types::Result<Self> {
        todo!()
    }

    fn new_number<N: Into<f64>>(value: N) -> crate::types::Result<Self> {
        todo!()
    }

    fn new_bool(value: bool) -> crate::types::Result<Self> {
        todo!()
    }

    fn new_null() -> crate::types::Result<Self> {
        todo!()
    }

    fn new_undefined() -> crate::types::Result<Self> {
        todo!()
    }

    fn new_function(func: &fn()) -> crate::types::Result<Self> {
        todo!()
    }
}

impl<'a> From<Local<'a, Value>> for V8Value<'a> {
    fn from(value: Local<'a, Value>) -> Self {
        V8Value(value)
    }
}
