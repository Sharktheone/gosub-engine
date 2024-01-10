use crate::web_executor::js::JSContext;
use crate::types::Result;

pub trait JSInterop {
    fn implement<C: JSContext>(ctx: &mut C) -> Result<()>;
}

pub trait WASMInterop {
    fn implement<C: >(ctx: &mut C) -> Result<()>;
}