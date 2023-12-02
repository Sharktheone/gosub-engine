use crate::js::v8::{V8Context, V8Engine, V8Object, V8Value};

pub type Runtime = V8Engine;
pub type Context = V8Context;
pub type Object = V8Object;
pub type Value = V8Value;