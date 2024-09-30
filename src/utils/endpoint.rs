/// Macro to define a new endpoint
macro_rules! generate_endpoint {
    (
        $(#[$meta:meta])*
        fn $fn_name:ident;
        method: $method:ident;
        path: $path:literal;
        $(params: {
            $($param_name:ident : $param_type:ty),* $(,)?
        })?;
        $call_fn:expr
    ) => {
        $(#[$meta])*
        #[$method($path)]
        pub async fn $fn_name(
            state: actix_web::web::Data<crate::AppState>,
            $(
                $(
                    $param_name: $param_type,
                )*
            )?
        ) -> Result<actix_web::HttpResponse, crate::error::ServerResponseError>
        {
            ($call_fn)(state, $($($param_name),*)?).await
        }
    };
}

pub(crate) use generate_endpoint;
