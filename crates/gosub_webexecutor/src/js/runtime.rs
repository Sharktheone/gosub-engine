use gosub_shared::types::Result;

use crate::js::{Args, VariadicArgs, VariadicArgsInternal, WebArray, WebCompiled, WebContext, WebFunction, WebFunctionCallBack, WebFunctionCallBackVariadic, WebFunctionVariadic, WebGetterCallback, WebObject, WebObjectTemplate, WebSetterCallback, WebValue};
use crate::js::gc::GarbageCollectable;

// trait around the main JS engine (e.g V8, SpiderMonkey, JSC, etc.)
pub trait WebRuntime: Sized {
    type Context: WebContext<Self>;
    type Value: WebValue<Self>;
    type Object: WebObject<Self>;
    type TypedObject<I: GarbageCollectable>: WebObject<Self, I>;
    type ObjectTemplate: WebObjectTemplate<Self>;
    type Compiled: WebCompiled<Self>;
    type GetterCB: WebGetterCallback<Self>;
    type SetterCB: WebSetterCallback<Self>;
    type Function: WebFunction<Self>;
    type FunctionVariadic: WebFunctionVariadic<Self>;
    type Array: WebArray<Self>;
    type FunctionCallBack: WebFunctionCallBack<Self>;
    type FunctionCallBackVariadic: WebFunctionCallBackVariadic<Self>;
    type Args: Args<Self>;
    type VariadicArgs: VariadicArgs<Self>;
    type VariadicArgsInternal: VariadicArgsInternal<Self>;

    fn new_context(&mut self) -> Result<Self::Context>;
}
