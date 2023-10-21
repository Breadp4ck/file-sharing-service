use axum::{
    routing::{get, post},
    Router,
};

mod shared_file;

use shared_file::*;

use crate::model::ModelController;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .nest(
            "/files",
            Router::new()
                .route("/:id", get(download_shared_file))
                .route("/", post(create_shared_file))
                .route("/:id/metadata", get(read_shared_file)),
        )
        .with_state(mc)
}
