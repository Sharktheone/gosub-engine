use crate::js::{JSObject, JSValue};
use crate::js::compile::JSCompiled;

pub trait JSContext {
    type Object: JSObject;

    type Value: JSValue;

    type Compiled: JSCompiled;

    fn run(&mut self, code: &str) -> crate::types::Result<Self::Value>;

    fn compile(&mut self, code: &str) -> crate::types::Result<Self::Compiled>;

    fn run_compiled(&mut self, compiled: &mut Self::Compiled) -> crate::types::Result<Self::Value>;

    // fn compile_stream(&self, code: &str) -> Result<()>;

    fn new_global_object(&mut self, name: &str) -> crate::types::Result<Self::Object>;
}

pub struct Context<C: JSContext>(pub C);

impl<T> JSContext for Context<T>
where
    T: JSContext,
{
    type Object = T::Object;
    type Value = T::Value;

    type Compiled = T::Compiled;

    fn run(&mut self, code: &str) -> crate::types::Result<Self::Value> {
        self.0.run(code)
    }

    fn compile(&mut self, code: &str) -> crate::types::Result<Self::Compiled> {
        self.0.compile(code)
    }

    fn run_compiled(&mut self, compiled: &mut Self::Compiled) -> crate::types::Result<Self::Value> {
        self.0.run_compiled(compiled)
    }

    fn new_global_object(&mut self, name: &str) -> crate::types::Result<Self::Object> {
        self.0.new_global_object(name)
    }
}
