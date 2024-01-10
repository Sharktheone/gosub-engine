#![allow(unused, unused_imports, unused_variables, dead_code)]

extern crate proc_macro;

use proc_macro::TokenStream;
use std::cell::RefCell;
use std::collections::HashMap;

use syn::{FnArg, ItemImpl, ItemStruct};
use syn::__private::ToTokens;

use crate::implement::implement;
use crate::items::{Field, Function};
use crate::property::parse_property;
use crate::types::{parse_return, parse_type, SelfType};

mod items;
mod types;
mod property;
mod impl_function;
mod utils;
mod implement;


thread_local!(
    static STATE: RefCell<HashMap<String, (Vec<Field>, Vec<Function>)>> = RefCell::new(HashMap::new());
);

#[proc_macro_attribute]
pub fn web_interop(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut fields: Vec<Field> = Vec::new();

    let mut input: ItemStruct = syn::parse_macro_input!(item);

    for field in &mut input.fields {
        if let Some(property) = parse_property(&mut field.attrs) {
            let mut f = Field {
                name: property.rename.unwrap_or(field.ident.as_ref().unwrap().to_string()),
                executor: property.executor,
                field_type: parse_type(field.ty.clone(), false).unwrap(),
            };
        }
    }

    let name = input.ident.clone().into_token_stream().to_string();

    if STATE.with(|state| state.borrow().contains_key(&name)) {
        STATE.with(|state| {
            let state = state.borrow();
            let (_, functions) = state.get(&name).unwrap();

            implement(&fields, functions)
        });
    } else {
        STATE.with_borrow_mut(|state| {
            state.insert(name, (fields, Vec::new()));
        });
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

    let name = input.self_ty.clone().into_token_stream().to_string();

    for mut func in input.items {
        if let syn::ImplItem::Fn(mut method) = func {
            let args = method.sig.inputs;
            let mut self_type = SelfType::NoSelf;

            let property = parse_property(&mut method.attrs).unwrap_or_default();

            let mut func = Function {
                name: property.rename.unwrap_or(method.sig.ident.to_string()),
                arguments: vec![],
                return_type: parse_return(method.sig.output).unwrap(),
                executor: property.executor,
            };

            if let Some(FnArg::Receiver(self_arg)) = args.first() {
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

    if STATE.with(|state| state.borrow().contains_key(&name)) {
        STATE.with(|state| {
            let state = state.borrow();
            let (fields, _) = state.get(&name).unwrap();

            implement(&fields, &functions)
        });
    } else {
        STATE.with_borrow_mut(|state| {
            state.insert(name, (Vec::new(), functions));
        });
    }

    item
}
