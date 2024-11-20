use actix_web::HttpResponse;
use helper_macros::generate_endpoint;
use crate::auth::session::UserSession;

generate_endpoint! {
    fn revoke;
    method: get;
    path: "/revoke";
    docs: {
        tag: "oauth",
        responses: {
            (status = 200, description = "User logged out successfully"),
            (status = 401, description = "Not logged in"),
            (status = 404, description = "User not found or invalid credentials"),
            (status = 500, description = "An error occurred when deleting the session from the database"),
        },
        security: [
            ("bearer_token" = []),
            ("cookie_session" = []),
        ]
    }
    params: {
        session: UserSession
    };
    {
        session.delete().await?;

        Ok(HttpResponse::Ok().finish())
    }
}
