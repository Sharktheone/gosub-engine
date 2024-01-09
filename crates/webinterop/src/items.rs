use syn::{Path};
use crate::types::{FunctionArg, ReturnType, Type};

pub(crate) struct Function {
    pub(crate) name: String,
    pub(crate) arguments: Vec<FunctionArg>,
    pub(crate) return_type: ReturnType,
    pub(crate) executor: Executor,

}

pub(crate) struct Field {
    pub(crate) name: String,
    pub(crate) field_type: Type,
    pub(crate) executor: Executor,
}

#[derive(PartialEq, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum Executor {
    JS,
    WASM,
    Both,
    None,
}

