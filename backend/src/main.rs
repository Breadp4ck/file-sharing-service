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

pub async fn index_page() -> impl IntoResponse {
    Html(
        r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/" method="post" enctype="multipart/form-data">
                    <label for="name">
                        Enter your note:
                        <input type="text" name="note">
                    </label>

                    <label for="pass">
                        Enter your password:
                        <input type="password" name="pass">
                    </label>

                    <label>
                        Upload file:
                        <input type="file" name="file" multiple required>
                    </label>

                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
        "#,
    )
}

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

    let app = Router::new()
        .route("/", get(index_page))
        .with_state(mc.clone())
        .nest("/api", routes_api)
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
