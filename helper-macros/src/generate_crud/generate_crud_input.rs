use proc_macro2::TokenStream as TokenStream2;
use syn::{Attribute, Ident, ItemStruct, LitStr, Token};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use crate::generate_crud::crud_operation::CrudOperation;

pub(crate) struct GenerateCrudInput {
    struct_def: ItemStruct,
    pub(crate) table_name: LitStr,
    pub(crate) create_op: Option<CrudOperation>,
    pub(crate) read_op: Option<CrudOperation>,
    pub(crate) update_op: Option<CrudOperation>,
    pub(crate) delete_op: Option<CrudOperation>,
}

impl Parse for GenerateCrudInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Parse the struct definition
        let struct_def: ItemStruct = input.parse()?;

        // Expect 'table:' keyword
        let table_ident: Ident = input.parse()?;
        if table_ident != "table" {
            return Err(syn::Error::new_spanned(table_ident, "Expected 'table' keyword"));
        }
        input.parse::<Token![:]>()?;
        let table_name: LitStr = input.parse()?;
        input.parse::<Token![,]>()?;

        // Initialize CRUD operations as None
        let mut create_op = None;
        let mut read_op = None;
        let mut update_op = None;
        let mut delete_op = None;

        // Parse CRUD operations
        while !input.is_empty() {
            // Parse the operation keyword (create, read, update, delete)
            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) {
                let op_ident: Ident = input.parse()?;
                input.parse::<Token![:]>()?;
                let content;
                syn::braced!(content in input);

                let op = content.parse::<CrudOperation>()?;

                match op_ident.to_string().as_str() {
                    "create" => create_op = Some(op),
                    "read" => read_op = Some(op),
                    "update" => update_op = Some(op),
                    "delete" => delete_op = Some(op),
                    _ => {
                        return Err(syn::Error::new_spanned(
                            op_ident,
                            "Expected 'create', 'read', 'update', or 'delete'",
                        ));
                    }
                }
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(GenerateCrudInput {
            struct_def,
            table_name,
            create_op,
            read_op,
            update_op,
            delete_op,
        })
    }
}
