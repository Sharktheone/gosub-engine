use crate::js::v8::V8Engine;
use crate::js::{Context, JSContext};
use crate::types::Result;


//trait around the main JS engine (e.g V8, SpiderMonkey, JSC, etc.)
pub trait JSRuntime {
    type Context: JSContext;

    fn new_context(&mut self) -> Result<Context<Self::Context>>;
}

pub struct Runtime<R: JSRuntime>(pub R);

impl Runtime<V8Engine<'_>> {
    pub fn new() -> Self {
        Self(V8Engine::new())
    }
}

impl<R: JSRuntime> JSRuntime for Runtime<R> {
    type Context = R::Context;

    fn new_context(&mut self) -> Result<Context<Self::Context>> {
        self.0.new_context()
    }
}
