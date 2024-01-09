use crate::web_executor::js::JSContext;

pub trait JSFunctionInterop {
    fn implement<C: JSContext>(ctx: C);
}

pub trait WASMFunctionInterop {
    fn implement<C: >(ctx: C);
}

pub trait JSPropertyInterop {
    fn implement<C: JSContext>(ctx: C);
}

pub trait WASMPropertyInterop {
    fn implement<C: >(ctx: C);
}