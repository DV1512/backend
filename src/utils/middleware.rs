/// macro to define a middleware removing most of the boilerplate code for implementing a middleware
macro_rules! define_middleware {
    {
        $(#[$meta:meta])*
        $vis:vis struct $middleware_name:ident {
            $($middleware_field:ident : $middleware_type:ty),* $(,)?
        },

        $(#[$service_meta:meta])*
        $service_vis:vis struct $service_name:ident;

        $call_fn:expr
    } => {
        $(#[$meta])*
        $vis struct $middleware_name {
            $(pub $middleware_field: $middleware_type),*
        }

        crate::utils::middleware::implement_transform!(
            $middleware_name {
                $($middleware_field : $middleware_type),*
            },
            $service_name;
        );

        $(#[$service_meta])*
        $service_vis struct $service_name<S> {
            service: std::sync::Arc<S>,
            $($middleware_field: $middleware_type),*
        }

        crate::utils::middleware::implement_service!(
            $service_name,
            $call_fn
        );
    };
}

pub(crate) use define_middleware;

macro_rules! implement_transform {
    (
        $middleware_name:ident {
            $($middleware_field:ident : $middleware_type:ty),* $(,)?
        },
        $service_name:ident;

        $new_transform:block,

        $new_service:block
    ) => {
        impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for $middleware_name
        where
            S: actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error> + 'static,
            B: 'static,
        {
            type Response = S::Response;
            type Error = S::Error;
            type Transform = $service_name<S>;
            type InitError = ();
            type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

            fn new_transform(&self, service: S) -> Self::Future {
                $new_transform
            }
        }
    };

    (
        $middleware_name:ident {
            $($middleware_field:ident : $middleware_type:ty),* $(,)?
        },
        $service_name:ident;
    ) => {
        impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest> for $middleware_name
        where
            S: actix_web::dev::Service<actix_web::dev::ServiceRequest, Response = actix_web::dev::ServiceResponse<B>, Error = actix_web::Error> + 'static,
            B: 'static,
        {
            type Response = S::Response;
            type Error = S::Error;
            type Transform = $service_name<S>;
            type InitError = ();
            type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

            fn new_transform(&self, service: S) -> Self::Future {
                futures::future::ok($service_name {
                    service: std::sync::Arc::new(service),
                    $($middleware_field: self.$middleware_field.clone()),*
                })
            }
        }
    };
}
pub(crate) use implement_transform;

macro_rules! implement_service {
    (
        $service_name:ident,
        $call_fn:expr
    ) => {
        impl<S, B> actix_web::dev::Service<actix_web::dev::ServiceRequest> for $service_name<S>
        where
            S: actix_web::dev::Service<
                    actix_web::dev::ServiceRequest,
                    Response = actix_web::dev::ServiceResponse<B>,
                    Error = actix_web::Error,
                > + 'static,
            B: 'static,
        {
            type Response = S::Response;
            type Error = S::Error;
            type Future =
                futures::future::LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

            actix_web::dev::forward_ready!(service);

            fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
                ($call_fn)(self, req)
            }
        }
    };
}
pub(crate) use implement_service;
