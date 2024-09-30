#![cfg(test)]

use syn::__private::TokenStream2;
use syn::parse_quote;
use crate::generate_crud::generate_crud_input::GenerateCrudInput;

#[test]
fn test_parse_generate_crud_input() {
    let input: TokenStream2 = parse_quote! {
        pub struct User {
            pub id: Option<String>,
            pub name: String,
            pub email: String,
        }
        table: "users",

        create: {
            fn create_user;
            endpoint: create_user_endpoint;
            path: "/users";
            params: {
                #[derive(Serialize, Deserialize, Debug, Clone)]
                struct CreateUserParams {
                    name: String
                }
            };
            query: println!("Deleting user");
        }

        read: {
            fn get_user;
            endpoint: get_user_endpoint;
            path: "/users/{id}";
            query: println!("Deleting user");
        }

        update: {
            fn update_user;
            endpoint: update_user_endpoint;
            path: "/users/{id}";
            query: println!("Deleting user");
        }

        delete: {
            fn delete_user;
            endpoint: delete_user_endpoint;
            path: "/users/{id}";
            query: println!("Deleting user");
        }
    };

    let parsed_input: GenerateCrudInput = syn::parse2(input).unwrap();

    assert_eq!(parsed_input.table_name.value(), "users");
    assert!(parsed_input.create_op.is_some());
    assert!(parsed_input.read_op.is_some());
    assert!(parsed_input.update_op.is_some());
    assert!(parsed_input.delete_op.is_some());
}
