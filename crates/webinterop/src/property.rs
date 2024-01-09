use syn::{Attribute, LitStr, Meta};
use crate::items::{Executor, Field};

pub(crate) struct Property {
    pub(crate) rename: Option<String>,
    pub(crate) executor: Executor,
}

pub(crate) fn parse_property(attrs: &mut Vec<Attribute>) -> Option<Property> {
    let mut remove_attrs = None;
    let mut property = None;

    for (index, attr) in attrs.iter().enumerate() {
        if attr.path().is_ident("property") {
            property = Some(Property {
                rename: None,
                executor: Executor::Both,
            });


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

                                property.as_mut().unwrap().rename = Some(lit.value());
                            }
                            path if path.is_ident("js") => {
                                if property.as_mut().unwrap().executor != Executor::Both {
                                    panic!("Executor cannot be specified twice!")
                                }
                                property.as_mut().unwrap().executor = Executor::JS;
                            }
                            path if path.is_ident("wasm") => {
                                if property.as_mut().unwrap().executor != Executor::Both {
                                    panic!("Executor cannot be specified twice!")
                                }
                                property.as_mut().unwrap().executor = Executor::WASM;
                            }
                            path if path.is_ident("none") => {
                                if property.as_mut().unwrap().executor != Executor::Both {
                                    panic!("Executor cannot be specified twice!")
                                }
                                property.as_mut().unwrap().executor = Executor::None;
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


            remove_attrs = Some(index);


        }
    }

    if let Some(index) = remove_attrs {
        attrs.remove(index);
    }

    property
}