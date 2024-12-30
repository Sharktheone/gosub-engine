use std::cell::RefCell;
use std::rc::Rc;
use gosub_webexecutor::js::{JSInterop, JSRuntime};
use gosub_shared::types::Result;

mod console;




pub fn implement<RT: JSRuntime>(ctx: RT::Context) -> Result<()> {
    console::Console::implement::<RT>(Rc::new(RefCell::new(console::Console {})), ctx)?;
    
    
    Ok(())
    
}