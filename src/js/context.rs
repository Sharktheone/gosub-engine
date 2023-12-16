use crate::js::{JSObject, JSValue};

pub trait JSContext {
    type Object: JSObject;

    type Value: JSValue;

    fn run(&self, code: &str) -> crate::types::Result<Self::Value>;

    fn compile(&self, code: &str) -> crate::types::Result<()>;

    fn run_compiled(&self) -> crate::types::Result<Self::Value>;

    // fn compile_stream(&self, code: &str) -> Result<()>;

    fn new_global_object(&self, name: &str) -> crate::types::Result<Self::Object>;
}

pub struct Context<C: JSContext>(pub C);

impl<T> JSContext for Context<T>
where
    T: JSContext,
{
    type Object = T::Object;
    type Value = T::Value;

    fn run(&self, code: &str) -> crate::types::Result<Self::Value> {
        self.0.run(code)
    }

    fn compile(&self, code: &str) -> crate::types::Result<()> {
        self.0.compile(code)
    }

    fn run_compiled(&self) -> crate::types::Result<Self::Value> {
        self.0.run_compiled()
    }

    fn new_global_object(&self, name: &str) -> crate::types::Result<Self::Object> {
        self.0.new_global_object(name)
    }
}
