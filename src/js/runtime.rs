use crate::js::context::Context;
use crate::js::{JSContext};
use crate::js::v8::V8Engine;
use crate::types::Result;


pub trait JSRuntime {
    type Context: JSContext;

    fn new_context(&self) -> Result<Context<Self::Context>>;
}


pub struct Runtime<R: JSRuntime>(R);


impl Runtime<V8Engine> {
    pub fn new() -> Self {
        Self(V8Engine::new())
    }
}

impl<R: JSRuntime> JSRuntime for Runtime<R> {
    type Context = R::Context;

    fn new_context(&self) -> Result<Context<Self::Context>> {
        self.0.new_context()
    }
}