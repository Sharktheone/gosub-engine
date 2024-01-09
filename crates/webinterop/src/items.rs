use syn::{Path, Type};
use crate::types::{FunctionArg, ReturnType};

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

#[derive(PartialEq)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum Executor {
    JS,
    WASM,
    Both,
    None,
}

