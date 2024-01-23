use crate::items::Executor;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::Path;

pub(crate) struct Type {
    pub(crate) reference: Reference,
    pub(crate) ty: TypeT,
}

pub(crate) enum ReturnType {
    Undefined,
    Type(TypeT),
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

pub(crate) fn parse_type(ty: syn::Type, allow_ref: bool) -> Result<Type, &'static str> {
    match ty {
        syn::Type::Reference(r) => {
            if !allow_ref {
                return Err("type can't be a reference here");
            }

            Ok(Type {
                reference: if r.mutability.is_none() {
                    Reference::Ref
                } else {
                    Reference::MutRef
                },
                ty: parse_type(*r.elem, false)
                    .map_err(|_| "double references not supported!")?
                    .ty,
            })
        }
        syn::Type::Array(a) => Ok(Type {
            reference: Reference::None,
            ty: TypeT::Array(Box::new(parse_type(*a.elem, allow_ref)?)),
        }),
        syn::Type::Slice(s) => Ok(Type {
            reference: Reference::None,
            ty: TypeT::Array(Box::new(parse_type(*s.elem, allow_ref)?)),
        }),
        syn::Type::Tuple(t) => {
            let mut elements = Vec::with_capacity(t.elems.len());

            for elem in t.elems {
                elements.push(parse_type(elem, allow_ref)?)
            }

            Ok(Type {
                reference: Reference::None,
                ty: TypeT::Tuple(elements),
            })
        }

        syn::Type::Path(p) => Ok(Type {
            reference: Reference::None,
            ty: TypeT::Type(p.path),
        }),
        _ => {
            panic!("Invalid argument type");
        }
    }
}

pub(crate) fn parse_return(ret: syn::ReturnType) -> Result<ReturnType, &'static str> {
    Ok(match ret {
        syn::ReturnType::Default => ReturnType::Undefined,
        syn::ReturnType::Type(_, ty) => ReturnType::Type(
            parse_type(*ty, false)
                .map_err(|_| "return type can't be a reference")?
                .ty,
        ),
    })
}
