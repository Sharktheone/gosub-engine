use crate::js::v8::V8Value;
use crate::js::JSArray;

pub struct V8Array<'a>(v8::Local<'a, v8::Array>);

impl<'a> JSArray for V8Array<'a> {
    type Value = V8Value<'a>;

    fn get(&self, index: usize) -> crate::types::Result<Self::Value> {
        todo!()
    }

    fn set(&self, index: usize, value: &str) -> crate::types::Result<()> {
        todo!()
    }

    fn push(&self, value: &str) -> crate::types::Result<()> {
        todo!()
    }

    fn pop(&self) -> crate::types::Result<Self::Value> {
        todo!()
    }

    fn length(&self) -> crate::types::Result<usize> {
        todo!()
    }
}
