use v8::{Handle, Local, Object};

use crate::js::{JSArray, JSObject};
use crate::js::v8::V8Value;

pub struct V8Object<'a>(Local<'a, Object>);

impl<'a> JSObject for V8Object<'a> {
    type Value = V8Value<'a>;

    fn set_property(&self, name: &str, value: &str) -> crate::types::Result<()> {
        todo!()
    }

    fn get_property(&self, name: &str) -> crate::types::Result<Self::Value> {
        todo!()
    }

    fn call_method(&self, name: &str, args: &[&str]) -> crate::types::Result<Self::Value> {
        todo!()
    }

    fn set_method(&self, name: &str, function: &str) -> crate::types::Result<()> {
        todo!()
    }
}


impl<'a> From<Local<'a, Object>> for V8Object<'a> {
    fn from(object: Local<'a, Object>) -> Self {
        Self(object)
    }
}