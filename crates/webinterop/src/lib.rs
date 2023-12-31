extern crate proc_macro;

use proc_macro::TokenStream;
use quote::__private::ext::RepToTokensExt;

use syn::{ItemStruct, LitStr, Meta};
use syn::__private::ToTokens;

use crate::items::{Executor, Field, Function, PropertyOptions};

mod items;

#[proc_macro_attribute]
pub fn web_interop(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fields: Vec<Field> = Vec::new();
    let mut functions: Vec<Function> = Vec::new();

    let mut input: ItemStruct = syn::parse_macro_input!(item);

    for field in &mut input.fields {
        let mut remove_attrs = Vec::new();

        for (index, attr) in field.attrs.iter().enumerate() {
            if attr.path().is_ident("property") {
                let mut options = PropertyOptions {
                    executor: Executor::Both,
                    rename: None,
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

                                    options.rename = Some(lit.value());
                                }
                                path if path.is_ident("js") => {
                                    options.executor = Executor::JS;
                                }
                                path if path.is_ident("wasm") => {
                                    options.executor = Executor::WASM;
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
            }
        }

        for index in remove_attrs {
            field.attrs.remove(index);
        }
    }

    input.into_token_stream().into()
}