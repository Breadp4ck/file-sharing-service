mod error;
pub mod shared_file;

pub use error::{Error, Result};
pub use shared_file::*;
use sqlx::postgres::{PgPool, PgPoolOptions};

use crate::config;

#[derive(Clone)]
pub struct ModelController {
    db: PgPool,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        let db_url = &config::config().db_url;

        let pool = match PgPoolOptions::new()
            .max_connections(10)
            .connect(db_url)
            .await
        {
            Ok(pool) => {
                log::info!("Connection to the database is successful!");
                pool
            }
            Err(err) => {
                log::error!("Failed to connect to the database: {:?}", err);
                std::process::exit(1);
            }
        };

        Ok(Self { db: pool })
    }

    pub(in crate::model) fn db(&self) -> &PgPool {
        &self.db
    }
}
