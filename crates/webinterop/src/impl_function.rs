use crate::items::{Executor, Function};
use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemImpl;

pub fn impl_js_functions(functions: Vec<Function>) -> TokenStream {
    let mut impls = Vec::new();
    for function in functions {
        assert_eq!(function.executor, Executor::JS);

        impls.push(impl_js_function(function));
    }
    quote! {
        #(#impls)*
    }
}
fn impl_js_function(function: Function) -> TokenStream {
    assert_eq!(function.executor, Executor::JS);

    let name = function.name;

    quote! {

        let

    }
}
