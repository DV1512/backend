use proc_macro::TokenStream;
use crate::generate_endpoint::generate_endpoint_internal;
use proc_macro_error::proc_macro_error;
use crate::generate_crud::generate_crud_internal;

mod generate_endpoint;
mod generate_crud;

#[proc_macro_error]
#[proc_macro]
/// Input to the `generate_endpoint` macro
///
/// # Example
///
/// ```rust
/// # use helper_macros::generate_endpoint;
/// generate_endpoint! {
///     fn login;
///     method: get;
///     path: "/health";
///     docs: {
///         tag: "health",
///         context_path: "/",
///         responses: {
///             (status = 200, description = "Everything works just fine!")
///         }
///     }
///     {
///         Ok(HttpResponse::Ok().body("Everything works just fine!"))
///     }
/// }
///```
///
pub fn generate_endpoint(input: TokenStream) -> TokenStream {
    generate_endpoint_internal(input.into()).into()
}

#[proc_macro_error]
#[proc_macro]
pub fn generate_crud(input: TokenStream) -> TokenStream {
    generate_crud_internal(input.into()).into()
}