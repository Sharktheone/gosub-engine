use std::cell::RefCell;
use std::rc::Rc;

use gosub_shared::types::Result;

use crate::js::WebRuntime;

pub trait JSInterop<RT: WebRuntime> {
    type Template: InteropTemplate<RT>;
    
    
    fn inherit_from(&self, parent: impl JSInterop<RT>) -> Result<()>;
    fn make_template() -> Self::Template;
    
    fn implement(self, t: Self::Template, ctx: RT::Context) -> Result<RT::Object>;
}


pub trait InteropTemplate<RT: WebRuntime> {
    fn get_template(&self) -> RT::ObjectTemplate;
}