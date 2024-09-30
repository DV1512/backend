use proc_macro2::TokenStream;
use proc_macro::TokenStream as TokenStream1;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, Block, Ident, LitStr, Token, Type,
};

pub(crate) fn generate_endpoint_internal(input: TokenStream) -> TokenStream1 {
    let input: TokenStream1 = input.into();

    let input = parse_macro_input!(input as GenerateEndpointInput).into();

    let GenerateEndpointInput {
        attrs,
        fn_name,
        method,
        path,
        params,
        fn_block,
    } = input;

    // Map method to the corresponding actix-web attribute
    let method_attr = match method.to_string().to_lowercase().as_str() {
        "get" => quote! { #[actix_web::get(#path)] },
        "post" => quote! { #[actix_web::post(#path)] },
        "put" => quote! { #[actix_web::put(#path)] },
        "delete" => quote! { #[actix_web::delete(#path)] },
        _ => {
            return syn::Error::new_spanned(
                method,
                "Unsupported method. Expected one of: get, post, put, delete.",
            )
            .to_compile_error()
            .into();
        }
    };

    // Generate function parameters
    let fn_params = if let Some(params) = params.clone() {
        let params_iter = params.iter().map(|p| {
            let name = &p.name;
            let ty = &p.ty;
            quote! { #name: #ty }
        });
        quote! { , #( #params_iter ),* }
    } else {
        quote! {}
    };

    // Generate the function
    let expanded = quote! {
        #(#attrs)*
        #method_attr
        pub async fn #fn_name(
            state: actix_web::web::Data<crate::AppState>
            #fn_params
        ) -> Result<actix_web::HttpResponse, crate::error::ServerResponseError>
        #fn_block
    };

    TokenStream1::from(expanded)
}

pub(crate) struct GenerateEndpointInput {
    attrs: Vec<Attribute>,
    fn_name: Ident,
    method: Ident,
    path: LitStr,
    params: Option<Vec<Parameter>>,
    fn_block: Block,
}

#[derive(Clone)]
struct Parameter {
    pub(crate) name: Ident,
    pub(crate) ty: Type,
}

impl Parse for GenerateEndpointInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse outer attributes
        let attrs = input.call(Attribute::parse_outer)?;

        // Parse 'fn' keyword and function name
        input.parse::<Token![fn]>()?;
        let fn_name: Ident = input.parse()?;
        input.parse::<Token![;]>()?;

        // Parse 'method: METHOD;'
        input.parse::<Ident>()?; // method
        input.parse::<Token![:]>()?;
        let method: Ident = input.parse()?;
        input.parse::<Token![;]>()?;

        // Parse 'path: "PATH";'
        input.parse::<Ident>()?; // path
        input.parse::<Token![:]>()?;
        let path: LitStr = input.parse()?;
        input.parse::<Token![;]>()?;

        // Optionally parse parameters
        let params = if input.peek(Ident) && input.peek2(Token![:]) {
            input.parse::<Ident>()?; // params
            input.parse::<Token![:]>()?;
            let content;
            syn::braced!(content in input);
            let params_punct = Punctuated::<Parameter, Token![,]>::parse_terminated(&content)?;
            input.parse::<Token![;]>()?;
            Some(params_punct.into_iter().collect())
        } else {
            None
        };

        // Parse the call function expression
        let fn_block: Block = input.parse()?;

        Ok(GenerateEndpointInput {
            attrs,
            fn_name,
            method,
            path,
            params,
            fn_block,
        })
    }
}

impl Parse for Parameter {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(Parameter { name, ty })
    }
}
