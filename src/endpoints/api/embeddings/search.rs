use actix_web::{web, HttpResponse};
use helper_macros::generate_endpoint;

use crate::{
    dto::embeddings::SearchEmbeddingsRequest, services::embeddings::search::search_embeddings_,
    state::AppState,
};

generate_endpoint! {
    fn search_embeddings;
    method: post;
    path: "/search";
    docs: {
        params: (),
        tag: "embeddings",
        responses: {
            (status = 201, description = "Embeddings created"),
            (status = 500, description = "Internal server error"),
        },
    }
    params: {
        data: web::Json<SearchEmbeddingsRequest>,
        state: web::Data<AppState>,
    };
    {
        let data = data.into_inner();
        let embeddings = search_embeddings_(
            &state.db,
            data.embedding,
            data.entry_type,
            data.num_neighbors,
        )
        .await?;
        Ok(HttpResponse::Ok().json(embeddings))
    }
}
