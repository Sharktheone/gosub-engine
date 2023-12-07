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