use crate::web_executor::js::v8::V8Engine;
use crate::web_executor::js::{Args, JSArray, JSCompiled, JSContext, JSFunction, JSFunctionCallBack, JSFunctionCallBackVariadic, JSFunctionVariadic, JSObject, JSValue, VariadicArgs};
use crate::types::Result;

//trait around the main JS engine (e.g V8, SpiderMonkey, JSC, etc.)
pub trait JSRuntime {
    type Array: JSArray;
    type Function: JSFunction;
    type FunctionVariadic: JSFunctionVariadic;
    type CB: JSFunctionCallBack;
    type VariadicCB: JSFunctionCallBackVariadic;
    type Compiled: JSCompiled;
    type Context: JSContext;
    type Value: JSValue;
    type Object: JSObject;
    type Args: Args;
    type VariadicArgs: VariadicArgs;

    fn new_context(&mut self) -> Result<Self::Context>;
}

