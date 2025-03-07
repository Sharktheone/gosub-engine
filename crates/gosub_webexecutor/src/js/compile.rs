use gosub_shared::types::Result;

use crate::js::WebRuntime;

//compiled code will be stored with this trait for later execution (e.g HTML parsing not done yet)
pub trait WebCompiled<RT: WebRuntime> {
    fn run(&mut self) -> Result<RT::Value>;
}
