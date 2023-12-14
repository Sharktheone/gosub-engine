use crate::js::{JSObject};


pub trait JSContext {

    type Object: JSObject;

    fn run(&self, code: &str) -> crate::types::Result<()>;

    fn compile(&self, code: &str) -> crate::types::Result<()>;

    fn run_compiled(&self) -> crate::types::Result<()>;

    // fn compile_stream(&self, code: &str) -> Result<()>;

    fn add_global_object(&self, name: &str) -> crate::types::Result<Self::Object>;
}

pub struct Context<C: JSContext>(pub C);


impl<T> JSContext for Context<T>
where T: JSContext
{
    type Object = T::Object;

    fn run(&self, code: &str) -> crate::types::Result<()> {
        self.0.run(code)
    }

    fn compile(&self, code: &str) -> crate::types::Result<()> {
        self.0.compile(code)
    }

    fn run_compiled(&self) -> crate::types::Result<()> {
        self.0.run_compiled()
    }

    fn add_global_object(&self, name: &str) -> crate::types::Result<Self::Object> {
        self.0.add_global_object(name)
    }
}