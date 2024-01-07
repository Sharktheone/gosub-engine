use syn::Path;
use crate::items::Executor;

pub(crate) struct Type {
    pub(crate) reference: Reference,
    pub(crate) ty: TypeT,
}


pub(crate) enum ReturnType {
    Undefined,
    Type(TypeT)
}


pub(crate) enum TypeT {
    None,
    Type(Path),
    Array(Box<Type>),
    Tuple(Vec<Type>), //Array on the JS side
}

pub(crate) enum SelfType {
    NoSelf,
    SelfRef,
    SelfMutRef,
}

pub(crate) enum Reference {
    None,
    Ref,
    MutRef,
}

pub(crate) struct PropertyOptions {
    pub(crate) executor: Executor,
    pub(crate) rename: Option<String>,
}

pub(crate) struct FunctionArg {
    pub(crate) reference: Reference,
    pub(crate) ty: TypeT,
}