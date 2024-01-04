extern crate proc_macro;

use proc_macro::TokenStream;
use std::env;

use syn::{ItemImpl, ItemStruct, LitStr, Meta, Path};
use syn::__private::ToTokens;

use crate::items::{Executor, Field, Function};

mod items;
mod js_types;

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

    let mut input: ItemImpl = syn::parse_macro_input!(item2);

    dbg!(input.clone());

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