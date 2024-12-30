use gosub_webinterop::{web_fns, web_interop};
use gosub_webexecutor::js::{
    Args, IntoJSValue, IntoRustValue, JSContext, JSFunction, JSFunctionCallBack, JSFunctionCallBackVariadic,
    JSFunctionVariadic, JSGetterCallback, JSInterop, JSObject, JSRuntime, JSSetterCallback, JSValue, VariadicArgs,
    VariadicArgsInternal,
};
use gosub_shared::types::Result;

use std::cell::RefCell;
use std::rc::Rc;
use log::info;

#[web_interop(rename(console))]
pub struct Console {}



#[web_fns(1)]
impl Console {
    fn log(args: &impl VariadicArgs) {
        let mut out = String::new();
        
        for arg in args.as_vec() {
            out.push_str(&arg.as_string().unwrap());
            out.push(' ');
        }
        
        out.pop();
        
        println!("{}", out);
    }
}