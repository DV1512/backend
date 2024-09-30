use proc_macro::TokenStream;
use crate::generate_endpoint::generate_endpoint_internal;
use proc_macro_error::proc_macro_error;

mod generate_endpoint;
mod generate_crud;

#[proc_macro_error]
#[proc_macro]
pub fn generate_endpoint(input: TokenStream) -> TokenStream {
    generate_endpoint_internal(input)
}
