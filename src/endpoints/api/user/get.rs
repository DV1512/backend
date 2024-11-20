use actix_web::web;
use tracing::info;
use helper_macros::generate_endpoint;
use crate::auth::UserInfoExampleResponses;
use crate::auth::users::get::{get_user_by_internal, GetUserBy};
use crate::extractors::Authenticated;
use crate::state::AppState;

generate_endpoint! {
    fn get_user_by;
    method: get;
    path: "";
    docs: {
        params: (GetUserBy),
        tag: "user",
        responses: {
            (status = 200, response = UserInfoExampleResponses),
            (status = 401, description = "Invalid credentials"),
            (status = 404, description = "User not found"),
        },
        security: [
            ("bearer_token" = []),
            ("cookie_session" = []),
        ]
    }
    params: {
        _auth: Authenticated,
        state: web::Data<AppState>,
        data: web::Query<GetUserBy>,
    };
    {
        info!("Retrieving user");
        let data = data.into_inner();
        let db = &state.db;
        let user = get_user_by_internal(db, &data).await?;

        Ok(web::Json(user))
    }
}