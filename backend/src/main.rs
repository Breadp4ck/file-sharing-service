use std::{net::SocketAddr, path::Path};

use axum::{
    http::{HeaderValue, Method},
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use model::ModelController;
use tower_http::cors::{Any, CorsLayer};

mod config;
mod error;
mod model;
mod utils;
mod web;

pub use error::{Error, Result};

const UPLOADS_DIRECTORY: &str = "uploads";

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mc = ModelController::new().await?;

    if !Path::is_dir(&Path::new(UPLOADS_DIRECTORY)) {
        tokio::fs::create_dir(UPLOADS_DIRECTORY)
            .await
            .expect("Can't create upload directory");
    }

    let routes_api = web::api::routes(mc.clone());

    let cors = CorsLayer::new().allow_origin(Any).allow_methods([
        Method::GET,
        Method::POST,
        Method::PATCH,
        Method::DELETE,
    ]);

    let app = Router::new().nest("/api", routes_api).layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
