use crate::js::v8::V8Value;
use crate::js::{JSArray, JSObject};
use v8::Local;

pub struct V8Object<'a>(Local<'a, v8::Object>);

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
