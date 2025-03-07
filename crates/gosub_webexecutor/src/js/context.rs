use gosub_shared::types::Result;

use crate::js::WebRuntime;

//main trait for JS context (can be implemented for different JS engines like V8, SpiderMonkey, JSC, etc.)
pub trait WebContext<RT: WebRuntime>: Clone {
    fn run(&mut self, code: &str) -> Result<RT::Value>;

    fn compile(&mut self, code: &str) -> Result<RT::Compiled>;

    fn run_compiled(
        &mut self,
        compiled: &mut RT::Compiled,
    ) -> Result<RT::Value>;

    // fn compile_stream(&self, code: &str) -> Result<()>;

    // fn new_global_object(&mut self, name: &str) -> Result<<Self::RT as WebRuntime>::Object>;

    fn set_on_global_object(
        &mut self,
        name: &str, //TODO: this should be impl IntoWebValue
        value: RT::Value,
    ) -> Result<()>;
}
