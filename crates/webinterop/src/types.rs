use syn::Path;
use crate::items::Executor;

pub(crate) struct Type {
    pub(crate) reference: Reference,
    pub(crate) ty: TypeT,

}


pub(crate) enum TypeT {
    None,
    Type(Path),
    Slice(Path, usize),
    Array(Path),
    Tuple(Vec<Path>), //Array on the JS side
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