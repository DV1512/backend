use proc_macro2::TokenStream;
use proc_macro::TokenStream as TokenStream1;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{parenthesized, parse::{Parse, ParseStream}, parse_macro_input, Attribute, Block, Ident, LitInt, LitStr, Token, Type};

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
        docs
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

    let docs_attr = if let Some(docs) = docs {
        let context_path = docs.context_path;
        let tag = docs.tag;
        let responses = docs.responses;

        let context_path = if let Some(context_path) = context_path {
            quote! { context_path = #context_path }
        } else {
            quote! {}
        };

        let tag = if let Some(tag) = tag {
            quote! { tag = #tag }
        } else {
            quote! {}
        };

        let responses = if let Some(responses) = responses {
            let response_iter = responses.iter().map(|response| {
                let status_code = response.status_code;
                let description = response.description.as_ref().map_or(quote! {}, |desc| quote! { description = #desc });
                let response_ty = response.response.as_ref().map_or(quote! {}, |res| quote! { response = #res });
                quote! {
                    (status = #status_code, #description, #response_ty)
                }
            });
            quote! { responses(#( #response_iter ),*) }
        } else {
            quote! {}
        };

        quote! {
            #[utoipa::path(#method, path = #path, #context_path, #tag, #responses)]
        }
    } else {
        quote! {}
    };

    // Generate the function
    let expanded = quote! {
        #(#attrs)*
        #docs_attr
        #method_attr
        pub async fn #fn_name(
            state: actix_web::web::Data<crate::AppState>
            #fn_params
        ) -> Result<actix_web::HttpResponse, crate::error::ServerResponseError>
        #fn_block
    };

    println!("{expanded}");

    TokenStream1::from(expanded)
}

pub(crate) struct GenerateEndpointInput {
    attrs: Vec<Attribute>,
    fn_name: Ident,
    method: Ident,
    path: LitStr,
    params: Option<Vec<Parameter>>,
    docs: Option<Documentation>,
    fn_block: Block,
}

pub(crate) struct Documentation {
    context_path: Option<LitStr>,
    tag: Option<LitStr>,
    responses: Option<Vec<Response>>,
}

impl Parse for Documentation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut context_path: Option<LitStr> = None;
        let mut tag: Option<LitStr> = None;
        let mut responses: Option<Vec<Response>> = None;

        // Parse in a loop, allowing fields in any order
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![:]>()?;  // Expect a colon after each identifier

            match ident.to_string().as_str() {
                "context_path" => {
                    if context_path.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Duplicate context_path"));
                    }
                    context_path = Some(input.parse()?);
                },
                "tag" => {
                    if tag.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Duplicate tag"));
                    }
                    tag = Some(input.parse()?);
                },
                "responses" => {
                    if responses.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Duplicate responses"));
                    }
                    let content;
                    syn::braced!(content in input);  // Expect a block around the responses
                    let parsed_responses = Punctuated::<Response, Token![,]>::parse_terminated(&content)?;
                    responses = Some(parsed_responses.into_iter().collect());
                },
                _ => return Err(syn::Error::new_spanned(ident, "Unexpected field")),
            }

            // Optionally consume a comma if present
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Documentation { context_path, tag, responses })
    }
}


pub(crate) struct Response {
    status_code: u16,
    description: Option<LitStr>,
    response: Option<Ident>,
}

impl Parse for Response {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        println!("parsing response");
        let content;
        parenthesized!(content in input);
        dbg!(&content);

        // Parse the status field
        let status_ident: Ident = content.parse()?;
        if status_ident != "status" {
            return Err(syn::Error::new_spanned(status_ident, "Expected 'status'"));
        }
        content.parse::<Token![=]>()?;
        let status_code_lit: LitInt = content.parse()?;
        let status_code: u16 = status_code_lit.base10_parse()?;

        content.parse::<Token![,]>()?;

        // Initialize optional fields
        let mut description: Option<LitStr> = None;
        let mut response: Option<Ident> = None;

        // Parse either description or response, ensuring mutual exclusion
        while !content.is_empty() {
            let ident: Ident = content.parse()?;
            content.parse::<Token![=]>()?;

            match ident.to_string().as_str() {
                "description" => {
                    if description.is_some() || response.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Cannot have both 'description' and 'response'"));
                    }
                    description = Some(content.parse()?);
                }
                "response" => {
                    if description.is_some() || response.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Cannot have both 'description' and 'response'"));
                    }
                    response = Some(content.parse()?);
                }
                _ => return Err(syn::Error::new_spanned(ident, "Unexpected field")),
            }

            // Optionally consume a comma if present
            if content.peek(Token![,]) {
                content.parse::<Token![,]>()?;
            }
        }

        // Ensure at least one of `description` or `response` is present
        if description.is_none() && response.is_none() {
            return Err(syn::Error::new_spanned(status_ident, "Either 'description' or 'response' must be present"));
        }

        Ok(Response { status_code, description, response })
    }
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

        let docs: Option<Documentation> = if input.peek(Ident) && input.peek2(Token![:]) {
            let ident = input.parse::<Ident>()?; // docs

            if ident != "docs" {
                None
            } else {
                input.parse::<Token![:]>()?;

                let content;
                syn::braced!(content in input);

                Some(content.parse()?)
            }
        } else {
            None
        };

        // Optionally parse parameters
        let params = if input.peek(Ident) && input.peek2(Token![:]) {
            let ident = input.parse::<Ident>()?; // params

            if ident != "params" {
                None
            } else {
                input.parse::<Token![:]>()?;
                let content;
                syn::braced!(content in input);
                let params_punct = Punctuated::<Parameter, Token![,]>::parse_terminated(&content)?;
                input.parse::<Token![;]>()?;
                Some(params_punct.into_iter().collect())
            }
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
            docs,
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
