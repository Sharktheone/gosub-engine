use syn::{Path, Type};
use crate::types::{FunctionArg, TypeT};

pub(crate) struct Function {
    pub(crate) name: String,
    pub(crate) arguments: Vec<FunctionArg>,
    pub(crate) return_type: TypeT,
    pub(crate) executor: Executor,

}

pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) field_type: Type,
    pub(crate) executor: Executor,
}


pub(crate) enum Executor {
    JS,
    WASM,
    Both,
}

