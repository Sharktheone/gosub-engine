use syn::{ReturnType, Type};

pub(crate) struct Function {
    pub(crate) name: String,
    pub(crate) arguments: Vec<Type>,
    pub(crate) return_type: ReturnType,
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

pub(crate) struct PropertyOptions {
    pub(crate) executor: Executor,
    pub(crate) rename: Option<String>,
}