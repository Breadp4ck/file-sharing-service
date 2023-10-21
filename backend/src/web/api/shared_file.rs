use axum::{
    body::StreamBody,
    extract::{Multipart, Path, State},
    http::{header, StatusCode},
    response::IntoResponse,
    Json,
};
use uuid::Uuid;

use crate::model::{
    ModelController, SharedFile, SharedFileForCreate, SharedFileForDownload, SharedFileForRead,
};

pub async fn create_shared_file(
    State(mc): State<ModelController>,
    mut multipart: Multipart,
) -> impl IntoResponse {
    let mut shared_file_c = SharedFileForCreate::default();

    // check note, password, file
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();

        match name.as_str() {
            "file" => {
                let filename = field.file_name().unwrap_or("no filename").to_string();

                if !filename.is_empty() {
                    shared_file_c.filename = filename;
                    shared_file_c.data = field.bytes().await.unwrap().into();
                }
            }
            "note" => {
                let note = field.text().await.unwrap();
                if !note.is_empty() {
                    // Note size can be too big to be included into database
                    shared_file_c.note = Some(note);
                }
            }
            "pass" => {
                let raw_password = field.text().await.unwrap();
                if !raw_password.is_empty() {
                    shared_file_c.raw_password = Some(sha256::digest(raw_password));
                }
            }
            _ => (), // return bad request
        }
    }

    // If there is no file, then there is no need to save anything
    if shared_file_c.filename.is_empty() {
        return Err((StatusCode::NOT_ACCEPTABLE, "Specify filename".to_string()));
    }

    match SharedFile::create(&mc, &shared_file_c).await {
        Ok(_) => Ok(()),
        Err(err) => {
            return Err((
                StatusCode::NOT_ACCEPTABLE,
                format!("Specify filename {}", err),
            ));
        }
    }
}

pub async fn read_shared_file(
    State(mc): State<ModelController>,
    Path(id): Path<Uuid>,
    Json(shared_file_r): Json<SharedFileForRead>,
) -> impl IntoResponse {
    match SharedFile::read(&mc, id, &shared_file_r).await {
        Ok(file) => Ok(Json(file)),
        Err(err) => Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    }
}

pub async fn download_shared_file(
    State(mc): State<ModelController>,
    Path(id): Path<Uuid>,
    Json(shared_file_d): Json<SharedFileForDownload>,
) -> impl IntoResponse {
    match SharedFile::download(&mc, id, &shared_file_d).await {
        Ok(stream) => {
            let body = StreamBody::new(stream);

            let headers = [
                (header::CONTENT_TYPE, "charset=utf-8"),
                (
                    header::CONTENT_DISPOSITION,
                    "attachment; filename=\"kek.png\"",
                ),
            ];

            Ok((headers, body))
        }
        Err(err) => Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    }
}
