#![allow(unused, unused_imports, unused_variables, dead_code)]


extern crate proc_macro;

use proc_macro::TokenStream;
use std::env;

use syn::{FnArg, ItemImpl, ItemStruct, LitStr, Meta, Path};
use syn::__private::ToTokens;
use syn::token::Return;

use crate::items::{Executor, Field, Function,};
use crate::types::{FunctionArg, Reference, ReturnType, SelfType, Type, TypeT};

mod items;
mod types;
mod property;

#[proc_macro_attribute]
pub fn web_interop(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fields: Vec<Field> = Vec::new();

    let mut input: ItemStruct = syn::parse_macro_input!(item);

    for field in &mut input.fields {
        if let Some(property) = parse_property(&mut field.attrs) {
            let mut f = Field {
                name: property.rename.unwrap_or(field.ident.as_ref().unwrap().to_string()),
                executor: property.executor,
                field_type: field.ty.clone(),
            };
        }

        for index in remove_attrs {
            field.attrs.remove(index);
        }
    }

    //TODO: do something with the parsed fields

    input.into_token_stream().into()
}


#[proc_macro_attribute]
pub fn web_fns(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: ItemImpl = {
        let item = item.clone();
        syn::parse_macro_input!(item)
    };
    let mut functions: Vec<Function> = Vec::new();

    for mut func in input.items {
        if let syn::ImplItem::Fn(mut method) = func {
            let args = method.sig.inputs;
            let mut self_type = SelfType::NoSelf;

            let property = parse_property(&mut method.attrs);

            let mut func = Function {
                name: method.sig.ident.to_string(),
                arguments: vec![],
                return_type: parse_return(method.sig.output).unwrap(),
                executor: Executor::Both, //TODO
            };

            if let Some(FnArg::Receiver(self_arg)) =  args.first() {
                if self_arg.reference.is_none() {
                    panic!("Self must be a reference");
                }

                match self_arg.mutability {
                    Some(_) => self_type = SelfType::SelfRef,
                    None => self_type = SelfType::SelfMutRef,
                };
            }

            let mut arg_types = Vec::with_capacity(args.len()); // we don't know if the first is self, so no args.len() - 1

            for arg in args {
                if let FnArg::Typed(arg) = arg {
                    arg_types.push(parse_type(*arg.ty, true))
                }
            }
        }
    }

    //TODO: do something with the functions

    item
}

fn get_crate() -> Path {
    let mut name = env::var("CARGO_PKG_NAME").unwrap();
    if name == "gosub-engine" {
        name = "crate".to_string();
    }

    let name = name.replace('-', "_");

    syn::parse_str::<Path>(&name).unwrap()
}


fn parse_type(ty: syn::Type, allow_ref: bool) -> Result<Type, &'static str> {
    match ty {
        syn::Type::Reference(r) => {
            if !allow_ref {
                return Err("type can't be a reference here")
            }

            Ok(Type {
                reference: if r.mutability.is_none() {Reference::Ref} else {Reference::MutRef},
                ty: parse_type(*r.elem, false).map_err(|_| "double references not supported!")?.ty,
            })
        }
        syn::Type::Array(a) => {
            Ok(Type {
                reference: Reference::None,
                ty: TypeT::Array(Box::new(parse_type(*a.elem, allow_ref)?))
            })
        }
        syn::Type::Slice(s) => {
            Ok(Type {
                reference: Reference::None,
                ty: TypeT::Array(Box::new(parse_type(*s.elem, allow_ref)?))
            })
        }
        syn::Type::Tuple(t) => {
            let mut elements = Vec::with_capacity(t.elems.len());

            for elem in t.elems {
                elements.push(parse_type(elem, allow_ref)?)
            }

            Ok(Type {
                reference: Reference::None,
                ty: TypeT::Tuple(elements)
            })
        }
        syn::Type::Path(p) => {
            Ok(Type {
                reference: Reference::None,
                ty: TypeT::Type(p.path), //TODO
            })

        }
        _ => {
            panic!("Invalid argument type");
        }
    }
}


fn parse_return(ret: syn::ReturnType) -> Result<ReturnType, &'static str> {
    Ok(match ret {
        syn::ReturnType::Default => ReturnType::Undefined,
        syn::ReturnType::Type(_, ty) => ReturnType::Type(parse_type(*ty, false).map_err(|_| "return type can't be a reference")?.ty)
    })
}