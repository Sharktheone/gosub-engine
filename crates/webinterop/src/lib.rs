extern crate proc_macro;

use proc_macro::TokenStream;

use syn::__private::ToTokens;
use syn::{ItemStruct};

#[proc_macro_attribute]
pub fn web_interop(attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut input: ItemStruct = syn::parse_macro_input!(item);

    for field in &mut input.fields {
        let mut remove_attrs = Vec::new();

        for (index, attr) in field.attrs.iter().enumerate() {
            if attr.path().is_ident("property") {
                remove_attrs.push(index);
            }
        }

        for index in remove_attrs {
            field.attrs.remove(index);
        }
    }

    input.into_token_stream().into()
}
