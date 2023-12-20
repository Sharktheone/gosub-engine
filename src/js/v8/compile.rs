use alloc::rc::Rc;
use v8::{Local, Script};
use crate::js::compile::JSCompiled;
use crate::js::{Context};
use crate::js::v8::{Ctx, V8Context, V8Value};
pub struct V8Compiled<'a> {
    compiled: Local<'a, Script>,
    context: Ctx<'a>,
}


impl<'a> V8Compiled<'a> {
    pub(super) fn from_compiled(ctx: Ctx<'a>, compiled: Local<'a, Script>) -> Self {
        Self {
            context: ctx,
            compiled
        }
    }

}

impl<'a> JSCompiled for V8Compiled<'a> {
    type Value = V8Value<'a>;

    type Context = Context<Ctx<'a>>;

    fn run(&mut self) -> crate::types::Result<Self::Value> {

        let try_catch = &mut v8::TryCatch::new(self.context.borrow_mut().scope());

        let Some(value) = self.compiled.run(try_catch) else {
            return Err(V8Context::report_exception(try_catch));
        };

        Ok(V8Value::from_value(Rc::clone(&self.context), value))
    }
}