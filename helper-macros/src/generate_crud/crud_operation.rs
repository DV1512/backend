use syn::{
    Attribute, Expr, Ident, ItemStruct, LitStr, Token, Visibility,
};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;

pub(crate) struct CrudOperation {
    pub(crate) attrs: Vec<Attribute>,
    pub(crate) vis: Visibility,
    pub(crate) fn_name: Ident,
    pub(crate) endpoint_fn_name: Ident,
    pub(crate)  path: LitStr,
    pub(crate) params_struct: Option<ItemStruct>,
    pub(crate) query_expr: Option<Expr>,
    pub(crate) before_hook: Option<Expr>,
    pub(crate) after_hook: Option<Expr>,
}

impl Parse for CrudOperation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse attributes
        let attrs = input.call(Attribute::parse_outer)?;

        // Parse optional visibility
        let vis: Visibility = input.parse()?;

        // Parse 'fn' keyword and function name
        input.parse::<Token![fn]>()?;
        let fn_name: Ident = input.parse()?;
        input.parse::<Token![;]>()?;

        // Optionally parse documentation comments
        let _docs = input.call(Attribute::parse_outer)?;

        // Parse 'endpoint:' and endpoint function name
        let ident = input.parse::<Ident>()?; // 'endpoint'

        if ident != "endpoint" {
            return Err(syn::Error::new_spanned(ident, "Expected 'endpoint' keyword"));
        }

        input.parse::<Token![:]>()?;
        let endpoint_fn_name: Ident = input.parse()?;
        input.parse::<Token![;]>()?;

        // Parse 'path:' and path literal
        let ident = input.parse::<Ident>()?; // 'path'

        if ident != "path" {
            return Err(syn::Error::new_spanned(ident, "Expected 'path' keyword"));
        }

        input.parse::<Token![:]>()?;
        let path: LitStr = input.parse()?;
        input.parse::<Token![;]>()?;

        // Optionally parse 'params' struct
        let mut params_struct = None;
        if input.peek(Ident) && input.peek2(Token![:]) && input.peek3(Token![struct]) {
            let ident: Ident = input.parse()?; // 'params'
            if ident == "params" {
                input.parse::<Token![:]>()?;
                let item_struct: ItemStruct = input.parse()?;
                input.parse::<Token![;]>()?;
                params_struct = Some(item_struct);
            }
        }

        dbg!(&params_struct);

        // Optionally parse 'query:' expression
        let mut query_expr = None;
        if input.peek(Ident) && input.peek2(Token![:]) {
            let ident: Ident = input.parse()?;
            dbg!(&ident);
            if ident == "query" {
                input.parse::<Token![:]>()?;
                let expr: Expr = input.parse()?;
                input.parse::<Token![;]>()?;
                query_expr = Some(expr);
            } else {
                return Err(syn::Error::new_spanned(ident, "Expected 'query'"));
            }
        }

        // Optionally parse 'before_' and 'after_' hooks
        let mut before_hook = None;
        let mut after_hook = None;
        while input.peek(Ident) && input.peek2(Token![:]) {
            let hook_ident: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let expr: Expr = input.parse()?;
            input.parse::<Token![;]>()?;

            match hook_ident.to_string().as_str() {
                "before_create" | "before_read" | "before_update" | "before_delete" => {
                    before_hook = Some(expr);
                }
                "after_create" | "after_read" | "after_update" | "after_delete" => {
                    after_hook = Some(expr);
                }
                _ => {
                    return Err(syn::Error::new_spanned(
                        hook_ident,
                        "Expected 'before_create', 'after_create', etc.",
                    ));
                }
            }
        }

        Ok(CrudOperation {
            attrs,
            vis,
            fn_name,
            endpoint_fn_name,
            path,
            params_struct,
            query_expr,
            before_hook,
            after_hook,
        })
    }
}
