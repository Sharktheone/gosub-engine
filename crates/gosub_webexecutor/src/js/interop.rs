use std::cell::RefCell;
use std::rc::Rc;

use gosub_shared::types::Result;

use crate::js::WebRuntime;

pub trait JSInterop {
    type Template;
    
    
    fn make_template() -> Self::Template;
    
    fn implement<RT: WebRuntime>(self, t: Self::Template, ctx: RT::Context) -> Result<RT::Object>;
}
