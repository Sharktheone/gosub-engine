use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn web_interop(attr: TokenStream, item: TokenStream) -> TokenStream {

    item
}