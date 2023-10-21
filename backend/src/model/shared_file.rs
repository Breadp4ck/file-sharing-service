use std::path::Path;

use crate::model::ModelController;
use crate::model::{Error, Result};
use crate::utils;

use serde::{Deserialize, Serialize};
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct SharedFile {
    pub id: uuid::Uuid,
    pub filename: String,
    pub note: Option<String>,
    pub password: Option<String>,
    pub downloads: i32,

    // Datetime info is initialized in database
    pub created_at: Option<chrono::NaiveDateTime>,
    pub expired_at: Option<chrono::NaiveDateTime>,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Default)]
pub struct SharedFileForCreate {
    pub filename: String,
    pub note: Option<String>,
    pub raw_password: Option<String>,
    pub data: Vec<u8>,
}

#[derive(Deserialize)]
pub struct SharedFileForRead {
    pub raw_password: Option<String>,
}

#[derive(Deserialize)]
pub struct SharedFileForDownload {
    pub raw_password: Option<String>,
}

impl SharedFile {
    pub async fn create(
        mc: &ModelController,
        shared_file_c: &SharedFileForCreate,
    ) -> Result<uuid::Uuid> {
        let id = uuid::Uuid::new_v4();
        let password = shared_file_c
            .raw_password
            .as_ref()
            .map_or(None, |v| Some(sha256::digest(v)));

        let path = Path::new(crate::UPLOADS_DIRECTORY).join(id.to_string());
        utils::save_file(&path, &shared_file_c.data)
            .await
            .map_err(|err| Error::FileSaving(err.to_string()))?;

        // start database add transaction
        let _ = sqlx::query(
            r#"INSERT INTO files (id, filename, note, password) VALUES ($1, $2, $3, $4)"#,
        )
        .bind(id.clone())
        .bind(shared_file_c.filename.clone())
        .bind(shared_file_c.note.clone())
        .bind(password.clone())
        .execute(mc.db())
        .await
        .map_err(|err: sqlx::Error| Error::Sqlx(err))?;

        Ok(id)
    }

    pub async fn read(
        mc: &ModelController,
        id: Uuid,
        shared_file_r: &SharedFileForRead,
    ) -> Result<SharedFile> {
        let password = shared_file_r
            .raw_password
            .as_ref()
            .map_or(None, |v| Some(sha256::digest(v)));

        let query_result = sqlx::query_as!(
            SharedFile,
            r#"SELECT * FROM files WHERE id = $1 AND password = $2"#,
            id,
            password,
        )
        .fetch_one(&mc.db)
        .await;

        match query_result {
            Ok(file) => Ok(file),
            Err(err) => Err(Error::Sqlx(err)),
        }
    }

    pub async fn download(
        mc: &ModelController,
        id: Uuid,
        shared_file_d: &SharedFileForDownload,
    ) -> Result<ReaderStream<File>> {
        let password = shared_file_d
            .raw_password
            .as_ref()
            .map_or(None, |v| Some(sha256::digest(v)));

        let query_result = sqlx::query_as!(
            SharedFile,
            r#"SELECT * FROM files WHERE id = $1 AND password = $2"#,
            id,
            password,
        )
        .fetch_one(mc.db())
        .await;

        match query_result {
            Ok(file) => {
                let server_file = File::open(format!("uploads/{}", file.id.to_string()))
                    .await
                    .map_err(|_| Error::FileNotFound)?;

                let stream = ReaderStream::new(server_file);
                Ok(stream)
            }
            Err(err) => Err(Error::Sqlx(err)),
        }
    }
}
