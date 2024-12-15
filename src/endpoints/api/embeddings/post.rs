use actix_web::{post, web, HttpResponse, Responder};
use helper_macros::generate_endpoint;

use crate::{
    dto::embeddings::{AddEmbeddingsRequest, SearchEmbeddingsRequest},
    error::ServerResponseError,
    services::embeddings::{add::insert_embeddings, search::search_embeddings_},
    state::AppState,
};

generate_endpoint! {
    fn add_embeddings;
    method: post;
    path: "";
    docs: {
        params: (),
        tag: "embeddings",
        responses: {
            (status = 201, description = "Embeddings created"),
            (status = 500, description = "Internal server error"),
        },
    }
    params: {
        data: web::Json<AddEmbeddingsRequest>,
        state: web::Data<AppState>,
    };
    {
        let data = data.into_inner();
        insert_embeddings(&state.db, data.entries, data.entry_type).await?;
        Ok(HttpResponse::Created())
    }
}
