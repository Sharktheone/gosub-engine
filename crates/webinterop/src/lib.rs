#![allow(unused, unused_imports, unused_variables, dead_code)]


extern crate proc_macro;

use proc_macro::TokenStream;
use std::env;

use syn::{FnArg, ItemImpl, ItemStruct, LitStr, Meta, Path};
use syn::__private::ToTokens;

use crate::items::{Executor, Field, Function,};
use crate::types::{FunctionArg, Reference, SelfType, Type, TypeT};

mod items;
mod types;

#[proc_macro_attribute]
pub fn web_interop(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fields: Vec<Field> = Vec::new();
    let mut functions: Vec<Function> = Vec::new();


    let item2 = item.clone();
    let mut input: ItemStruct = syn::parse_macro_input!(item2);


    for field in &mut input.fields {
        let mut remove_attrs = Vec::new();

        for (index, attr) in field.attrs.iter().enumerate() {
            if attr.path().is_ident("property") {
                let mut f = Field {
                    name: field.ident.as_ref().unwrap().to_string(),
                    executor: Executor::Both,
                    field_type: field.ty.clone(),
                };


                //rename = "____", js => rename to name and it is a js only property
                //rename = "____", wasm => rename to name and it is a wasm only property
                //rename = "____" => rename to name and it is a property for both, js and wasm
                //js => name is the same and it is a js only property
                //wasm => name is the same and it is a wasm only property
                //<nothing> => name is the same and it is a property for both, js and wasm

                match &attr.meta {
                    Meta::Path(_) => {}
                    Meta::List(_) => {
                        attr.parse_nested_meta(|meta| {
                            match &meta.path {
                                path if path.is_ident("rename") => {
                                    let lit: LitStr = meta.value()?.parse()?;

                                    f.name = lit.value();
                                }
                                path if path.is_ident("js") => {
                                    f.executor = Executor::JS;
                                }
                                path if path.is_ident("wasm") => {
                                    f.executor = Executor::WASM;
                                }
                                _ => Err(syn::Error::new_spanned(attr, "Unknown attribute in property attribute"))?
                            }

                            Ok(())
                        }).unwrap();
                    }
                    Meta::NameValue(_) => {
                        panic!("Unexpected NameValue in property attribute");
                    }
                }


                remove_attrs.push(index);

                fields.push(f);
            }
        }

        for index in remove_attrs {
            field.attrs.remove(index);
        }
    }

    // let impls: ItemImpl = syn::parse_macro_input!(item);
    //
    // dbg!(impls);


    input.into_token_stream().into()
}


#[proc_macro_attribute]
pub fn web_fns(attr: TokenStream, item: TokenStream) -> TokenStream {
    let item2 = item.clone();

    dbg!(&item);


    let mut input: ItemImpl = syn::parse_macro_input!(item2);
    let mut functions: Vec<Function> = Vec::new();

    for func in input.items {
        if let syn::ImplItem::Fn(method) = func {
            let args = method.sig.inputs;
            let mut self_type = SelfType::NoSelf;

            let mut func = Function {
                name: method.sig.ident.to_string(),
                arguments: vec![],
                return_type: TypeT::None,
                executor: Executor::Both, //TODO
            };

            // if let syn::ReturnType::Type(.., ty) =  method.sig.output {
            //     match *ty {
            //         syn::Type::Reference(_) => {
            //             panic!("Can't return a reference")
            //         }
            //         syn::Type::Array(a) => {
            //             func.return_type = ReturnType::Array(*a.elem);
            //         }
            //         syn::Type::Slice(s) => {
            //             func.return_type = ReturnType::Array(*s.elem);
            //         }
            //         syn::Type::Tuple(t) => {
            //             let mut elems = Vec::with_capacity(t.elems.len());
            //             for elem in t.elems {
            //                 elems.push(elem);
            //             }
            //
            //             func.return_type = ReturnType::Tuple(elems);
            //         }
            //         syn::Type::Path(p) => {
            //             func.return_type = ReturnType::Type(p.path);
            //         }
            //         _ => {
            //             panic!("Invalid return type");
            //         }
            //     }
            // }

            dbg!(&args);

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

            //dbg!(arg_types[0]);

            // dbg!(output);
            // dbg!(name);
        }
    }

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
