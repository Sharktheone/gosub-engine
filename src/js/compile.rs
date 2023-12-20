use crate::js::{JSContext, JSValue};

pub trait JSCompiled {
    type Value: JSValue;

    type Context: JSContext;

    fn run(&mut self) -> crate::types::Result<Self::Value>;
}