use proc_macro::TokenStream as TokenStream1;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::punctuated::Punctuated;
use syn::{
    parenthesized,
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, Block, Ident, LitInt, LitStr, Token, Type,
};

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
        docs,
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
        quote! { #( #params_iter ),* }
    } else {
        quote! {}
    };

    let docs_attr = if let Some(docs) = docs {
        let context_path = docs.context_path;
        let tag = docs.tag;
        let responses = docs.responses;
        let doc_params = docs.params;

        // Create a vector for optional attributes, and only include non-empty tokens
        let mut doc_tokens = vec![];

        if let Some(context_path) = context_path {
            doc_tokens.push(quote! { context_path = #context_path });
        }

        if let Some(tag) = tag {
            doc_tokens.push(quote! { tag = #tag });
        }

        if let Some(responses) = responses {
            let response_iter = responses.iter().map(|response| {
                let status_code = response.status_code;
                let description = response
                    .description
                    .as_ref()
                    .map_or(quote! {}, |desc| quote! {, description = #desc });
                let response_ty = response
                    .response
                    .as_ref()
                    .map_or(quote! {}, |res| quote! {, response = #res });
                quote! {
                    (status = #status_code #description #response_ty)
                }
            });
            doc_tokens.push(quote! { responses(#( #response_iter ),*) });
        }

        if let Some(doc_params) = doc_params {
            doc_tokens.push(quote! { params( #( #doc_params ),* ) });
        }

        // Join the doc tokens with commas only between non-empty tokens
        quote! {
            #[utoipa::path(#method, path = #path, #( #doc_tokens ),*)]
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
            #fn_params
        ) -> Result<impl ::actix_web::Responder, crate::error::ServerResponseError>
        #fn_block
    };

    TokenStream1::from(expanded)
}

/// Endpoint input
///
/// # Examples
///
/// ```no_compile
/// #[allow(dead_code)]
/// fn my_endpoint;
/// method: get;
/// path: "/";
/// params: {
///     #[derive(Serialize, Deserialize, Debug, Clone)]
///     struct MyParams {
///         name: String
///     }
/// }
/// docs: {
///     context_path: "/api"
///     tag: "tag"
///     responses: {
///         (status = 200, description = "Request successful"),
///         (status = 404, description = "Not found")
///     }
/// }
/// {
///     HttpResponse::Ok().body("Hello, world!")
/// }
/// ```
///
pub(crate) struct GenerateEndpointInput {
    attrs: Vec<Attribute>,
    fn_name: Ident,
    method: Ident,
    path: LitStr,
    params: Option<Vec<Parameter>>,
    docs: Option<Documentation>,
    fn_block: Block,
}

/// Endpoint documentation
///
/// # Examples
///
/// ```no_compile
/// context_path = "/api"
/// tag = "tag"
/// responses: {
///     (status = 200, description = "Request successful"),
///     (status = 404, description = "Not found")
/// }
/// ```
///
pub(crate) struct Documentation {
    context_path: Option<LitStr>,
    tag: Option<LitStr>,
    responses: Option<Vec<Response>>,
    params: Option<Vec<Ident>>,
}

impl Parse for Documentation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut context_path: Option<LitStr> = None;
        let mut tag: Option<LitStr> = None;
        let mut responses: Option<Vec<Response>> = None;
        let mut params: Option<Vec<Ident>> = None;

        // Parse in a loop, allowing fields in any order
        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![:]>()?; // Expect a colon after each identifier

            match ident.to_string().as_str() {
                "context_path" => {
                    if context_path.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Duplicate context_path"));
                    }
                    context_path = Some(input.parse()?);
                }
                "tag" => {
                    if tag.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Duplicate tag"));
                    }
                    tag = Some(input.parse()?);
                }
                "responses" => {
                    if responses.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Duplicate responses"));
                    }
                    let content;
                    syn::braced!(content in input); // Expect a block around the responses
                    let parsed_responses =
                        Punctuated::<Response, Token![,]>::parse_terminated(&content)?;
                    responses = Some(parsed_responses.into_iter().collect());
                }
                "params" => {
                    let content;
                    parenthesized!(content in input);

                    let parsed_params = Punctuated::<Ident, Token![,]>::parse_terminated(&content)?;
                    params = Some(parsed_params.into_iter().collect());
                }
                _ => return Err(syn::Error::new_spanned(ident, "Unexpected field")),
            }

            // Optionally consume a comma if present
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Documentation {
            context_path,
            tag,
            responses,
            params,
        })
    }
}

/// Endpoint response documentation
///
/// # Examples
///
/// ```no_compile
/// (status = 200, description = "Request successful")
/// ```
///
pub(crate) struct Response {
    status_code: u16,
    description: Option<LitStr>,
    response: Option<Ident>,
}

impl Parse for Response {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        parenthesized!(content in input);

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
                        return Err(syn::Error::new_spanned(
                            ident,
                            "Cannot have both 'description' and 'response'",
                        ));
                    }
                    description = Some(content.parse()?);
                }
                "response" => {
                    if description.is_some() || response.is_some() {
                        return Err(syn::Error::new_spanned(
                            ident,
                            "Cannot have both 'description' and 'response'",
                        ));
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
            return Err(syn::Error::new_spanned(
                status_ident,
                "Either 'description' or 'response' must be present",
            ));
        }

        Ok(Response {
            status_code,
            description,
            response,
        })
    }
}

/// Represents a parameter in a function signature
///
/// # Examples
///
/// ```no_compile
/// id: i32
/// ```
///
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

        let mut method: Option<Ident> = None;
        let mut path: Option<LitStr> = None;
        let mut docs: Option<Documentation> = None;
        let mut params: Option<Vec<Parameter>> = None;

        let mut ident = input.parse::<Ident>()?;

        while !input.is_empty() {
            input.parse::<Token![:]>()?;
            match ident.to_string().as_str() {
                "method" => {
                    if method.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Duplicate method"));
                    }

                    let method_ident = input.parse::<Ident>()?;

                    match method_ident.to_string().as_str() {
                        "get" => method = Some(Ident::new("get", Span::call_site())),
                        "post" => method = Some(Ident::new("post", Span::call_site())),
                        "put" => method = Some(Ident::new("put", Span::call_site())),
                        "patch" => method = Some(Ident::new("patch", Span::call_site())),
                        "delete" => method = Some(Ident::new("delete", Span::call_site())),
                        _ => return Err(syn::Error::new_spanned(method_ident, "Invalid method")),
                    }
                }
                "path" => {
                    if path.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Duplicate path"));
                    }

                    let path_lit = input.parse::<LitStr>()?;

                    path = Some(path_lit);
                }
                "docs" => {
                    if docs.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Duplicate docs"));
                    }

                    let content;
                    syn::braced!(content in input);

                    docs = Some(content.parse()?);
                }
                "params" => {
                    if params.is_some() {
                        return Err(syn::Error::new_spanned(ident, "Duplicate params"));
                    }

                    let content;
                    syn::braced!(content in input);
                    let parsed_params =
                        Punctuated::<Parameter, Token![,]>::parse_terminated(&content)?;

                    params = Some(
                        parsed_params
                            .into_iter()
                            .map(|param| Parameter {
                                name: param.name,
                                ty: param.ty,
                            })
                            .collect(),
                    );
                }
                _ => return Err(syn::Error::new_spanned(ident, "Unexpected field")),
            }

            if input.peek(Token![;]) {
                input.parse::<Token![;]>()?;
            }

            // if the next token is not an identifier, we're done parsing and move on to the function body
            if !input.peek(Ident) {
                break;
            }

            ident = input.parse::<Ident>()?;
        }

        if method.is_none() {
            return Err(syn::Error::new_spanned(ident, "Missing 'method'"));
        }

        if path.is_none() {
            return Err(syn::Error::new_spanned(ident, "Missing 'path'"));
        }

        // Parse the call function expression
        let fn_block: Block = input.parse()?;

        Ok(GenerateEndpointInput {
            attrs,
            fn_name,
            method: method.unwrap(),
            path: path.unwrap(),
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
