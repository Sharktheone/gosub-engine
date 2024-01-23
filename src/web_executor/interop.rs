use crate::types::Result;
use crate::web_executor::js::{JSContext, JSRuntime};

pub trait JSInterop {
    fn implement<R: JSRuntime>(&mut self, ctx: &mut R::Context) -> Result<()>;
}

// pub trait WASMInterop {
//     fn implement<R: >(ctx: &mut R::Context) -> Result<()>;
// }
