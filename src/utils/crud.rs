macro_rules! generate_crud {
    (
        // Struct definition
        $(#[$struct_meta:meta])*
        $vis:vis struct $struct_name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_name:ident : $field_type:ty
            ),* $(,)?
        }
        table: $table_name:literal,
        // Create operation
        create: {
            $(#[$create_meta:meta])*
            fn $create_fn_name:ident;
            $(#[$create_docs:meta])?
            endpoint: $create_endpoint_fn_name:ident;
            path: $create_path:literal;
            $(before_create: $before_create_fn:expr;)?
            $(after_create: $after_create_fn:expr;)?
        }
        // Read operation
        read: {
            $(#[$read_meta:meta])*
            fn $read_fn_name:ident;
            $(#[$read_docs:meta])?
            endpoint: $read_endpoint_fn_name:ident;
            path: $read_path:literal;
            params: {
                $(#[$param_meta:meta])*
                struct $get_params_name:ident {
                    $(
                        $(#[$param_field_meta:meta])*
                        $param_name:ident : $param_type:ty
                    ),* $(,)?
                }
            }
            query: $read_query:expr;
            $(before_read: $before_read_fn:expr;)?
            $(after_read: $after_read_fn:expr;)?
        }
        // Update operation
        update: {
            $(#[$update_meta:meta])*
            fn $update_fn_name:ident;
            $(#[$update_docs:meta])?
            endpoint: $update_endpoint_fn_name:ident;
            path: $update_path:literal;
            params: {
                $(#[$update_param_meta:meta])*
                struct $update_params_name:ident {
                    $(
                        $(#[$update_param_field_meta:meta])*
                        $update_param_name:ident : $update_param_type:ty
                    ),* $(,)?
                }
            }
            query: $update_query:expr;
            $(before_update: $before_update_fn:expr;)?
            $(after_update: $after_update_fn:expr;)?
        }
        // Delete operation
        delete: {
            $(#[$delete_meta:meta])*
            fn $delete_fn_name:ident;
            $(#[$delete_docs:meta])?
            endpoint: $delete_endpoint_fn_name:ident;
            path: $delete_path:literal;
            params: {
                $(#[$delete_param_meta:meta])*
                struct $delete_params_name:ident {
                    $(
                        $(#[$delete_param_field_meta:meta])*
                        $delete_param_name:ident : $delete_param_type:ty
                    ),* $(,)?
                }
            }
            query: $delete_query:expr;
            $(before_delete: $before_delete_fn:expr;)?
            $(after_delete: $after_delete_fn:expr;)?
        }
    ) => {
        // Struct definition
        $(#[$struct_meta])*
        $vis struct $struct_name {
            pub id: Option<crate::models::thing::Thing>,
            $(
                $(#[$field_meta])*
                pub $field_name: $field_type
            ),*
        }

        // GetParams struct
        $(#[$param_meta])*
        $vis struct $get_params_name {
            $(
                $(#[$param_field_meta])*
                pub $param_name: $param_type
            ),*
        }

        // UpdateParams struct
        $(#[$update_param_meta])*
        $vis struct $update_params_name {
            $(
                $(#[$update_param_field_meta])*
                pub $update_param_name: $update_param_type
            ),*
        }

        // DeleteParams struct
        $(#[$delete_param_meta])*
        $vis struct $delete_params_name {
            $(
                $(#[$delete_param_field_meta])*
                pub $delete_param_name: $delete_param_type
            ),*
        }

        // Create function
        $(#[$create_meta])*
        pub async fn $create_fn_name<T>(
            db: &::std::sync::Arc<::surrealdb::Surreal<T>>,
            data: $struct_name,
        ) -> Result<$struct_name, crate::error::ServerResponseError>
        where
            T: ::surrealdb::Connection,
        {
            $(($before_create_fn)(db, &data)?;)?

            let result: Option<$struct_name> = db.create($table_name)
                .content(data)
                .await?;

            $(($after_create_fn)(db, &result)?;)?

            if let Some(result) = result {
                Ok(result)
            } else {
                Err(crate::error::ServerResponseError::InternalError("Error inserting into database".to_string()))
            }
        }

        // Read function
        $(#[$read_meta])*
        pub async fn $read_fn_name<T>(
            db: &::std::sync::Arc<::surrealdb::Surreal<T>>,
            params: &$get_params_name,
        ) -> Result<Vec<$struct_name>, crate::error::ServerResponseError>
        where
            T: ::surrealdb::Connection
        {
            $(($before_read_fn)(db, params)?;)?

            let query: ::tosic_utils::QueryBuilder<::tosic_utils::Select> = $read_query(params)?;

            let result: Vec<$struct_name> = query.run(db, 0).await?;

            $(($after_read_fn)(db, &result)?;)?

            Ok(result)
        }

        // Update function
        $(#[$update_meta])*
        pub async fn $update_fn_name<T>(
            db: &::std::sync::Arc<::surrealdb::Surreal<T>>,
            params: &$update_params_name,
            data: &$struct_name,
        ) -> Result<$struct_name, crate::error::ServerResponseError>
        where
            T: ::surrealdb::Connection,
        {
            $(($before_update_fn)(db, params, data)?;)?

            let query: ::tosic_utils::QueryBuilder<::tosic_utils::Update> = $update_query(params, data)?;

            let result: Option<$struct_name> = query.run(db, 0).await?;

            $(($after_update_fn)(db, &result)?;)?

            if let Some(data) = result {
                Ok(data)
            } else {
                Err(crate::error::ServerResponseError::NotFound)
            }
        }

        // Delete function
        $(#[$delete_meta])*
        pub async fn $delete_fn_name<T>(
            db: &::std::sync::Arc<::surrealdb::Surreal<T>>,
            params: &$delete_params_name,
        ) -> Result<(), crate::error::ServerResponseError>
        where
            T: ::surrealdb::Connection,
        {
            $(($before_delete_fn)(db, params)?;)?

            let query: ::tosic_utils::QueryBuilder<::tosic_utils::Delete> = $delete_query(params)?;

            let _: Option<crate::Record> = query.run(db, 0).await?;

            $(($after_delete_fn)(db)?;)?

            Ok(())
        }

        use actix_web::{get, post, put, delete};

        // Create endpoint
        crate::utils::endpoint::generate_endpoint!{
            $(#[$create_meta])*
            $(#[$create_docs])?
            fn $create_endpoint_fn_name;
            method: post;
            path: $create_path;
            params: {
                data: ::actix_web::web::Json<$struct_name>
            };
            async |state: actix_web::web::Data<crate::AppState>, data: ::actix_web::web::Json<$struct_name>| -> Result<::actix_web::HttpResponse, crate::error::ServerResponseError> {
                let data = data.into_inner();
                let db = &state.db;

                match $create_fn_name(db, data).await {
                    Ok(result) => Ok(::actix_web::HttpResponse::Ok().json(result)),
                    Err(err) => Err(err),
                }
            }
        }

        // Read endpoint
        crate::utils::endpoint::generate_endpoint!{
            $(#[$read_meta])*
            $(#[$read_docs])?
            fn $read_endpoint_fn_name;
            method: get;
            path: $read_path;
            params: {
                params: ::actix_web::web::Query<$get_params_name>
            };
            async |state: actix_web::web::Data<crate::AppState>, params: ::actix_web::web::Query<$get_params_name>| -> Result<::actix_web::HttpResponse, crate::error::ServerResponseError> {
                let params = params.into_inner();
                let db = &state.db;

                match $read_fn_name(db, &params).await {
                    Ok(result) => Ok(::actix_web::HttpResponse::Ok().json(result)),
                    Err(err) => Err(err),
                }
            }
        }

        // Update endpoint
        crate::utils::endpoint::generate_endpoint!{
            $(#[$update_meta])*
            $(#[$update_docs])?
            fn $update_endpoint_fn_name;
            method: put;
            path: $update_path;
            params: {
                params: ::actix_web::web::Path<$update_params_name>,
                data: ::actix_web::web::Json<$struct_name>
            };
            async |state: actix_web::web::Data<crate::AppState>, params: ::actix_web::web::Path<$update_params_name>, data: ::actix_web::web::Json<$struct_name>| -> Result<::actix_web::HttpResponse, crate::error::ServerResponseError> {
                let params = params.into_inner();
                let data = data.into_inner();
                let db = &state.db;

                match $update_fn_name(db, &params, &data).await {
                    Ok(result) => Ok(::actix_web::HttpResponse::Ok().json(result)),
                    Err(err) => Err(err),
                }
            }
        }

        // Delete endpoint
        crate::utils::endpoint::generate_endpoint!{
            $(#[$delete_meta])*
            $(#[$delete_docs])?
            fn $delete_endpoint_fn_name;
            method: delete;
            path: $delete_path;
            params: {
                params: ::actix_web::web::Path<$delete_params_name>
            };
            async |state: actix_web::web::Data<crate::AppState>, params: ::actix_web::web::Path<$delete_params_name>| -> Result<::actix_web::HttpResponse, crate::error::ServerResponseError> {
                let params = params.into_inner();
                let db = &state.db;

                match $delete_fn_name(db, &params).await {
                    Ok(_) => Ok(::actix_web::HttpResponse::NoContent().finish()),
                    Err(err) => Err(err),
                }
            }
        }
    };
}

pub(crate) use generate_crud;
