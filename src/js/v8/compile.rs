use crate::js::v8::{Ctx, FromContext, V8Context, V8Value};
use crate::js::{Context, JSCompiled};
use alloc::rc::Rc;
use v8::{Local, Script};
pub struct V8Compiled<'a> {
    compiled: Local<'a, Script>,
    context: Ctx<'a>,
}

impl<'a> FromContext<'a, Local<'a, Script>> for V8Compiled<'a> {
    fn from_ctx(ctx: Ctx<'a>, value: Local<'a, Script>) -> Self {
        Self {
            context: ctx,
            compiled: value,
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