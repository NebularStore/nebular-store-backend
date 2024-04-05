use std::fs;
use std::ops::Not;
use std::path::PathBuf;

use anyhow::Result;
use axum::extract::{DefaultBodyLimit, Multipart, Path};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Json, Router};
use axum::body::Bytes;
use axum::extract::multipart::{Field, MultipartError};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;
/*
/files
    /structure
        GET /:path
        GET /
    /serve
        GET /:path
    /repository
        /file
            DELETE /:path
        /dir
            DELETE /:path
            POST /:path
        POST /move/:path
        POST /rename/:path
 */
pub fn files_router() -> Router {
    Router::new()
        .nest(
            "/structure/",
            Router::new()
                .route("/*path", get(specified_structure))
                .route("/", get(general_structure)),
        )
        .nest(
            "/repository/",
            Router::new()
                .nest(
                    "/file/",
                    Router::new()
                        .route("/*path", delete(delete_file))
                        .route("/*path", post(upload_file))
                        .layer(DefaultBodyLimit::disable()),
                )
                .nest(
                    "/dir",
                    Router::new()
                        .route("/*path", delete(delete_dir))
                        .route("/*path", post(create_dir)),
                )
                .route("/move/*path", post(move_entry))
                .route("/rename/*path", post(rename_entry)),
        )
        .nest_service("/serve/", ServeDir::new("data/repository"))
}

async fn specified_structure(Path(path): Path<String>) -> impl IntoResponse {
    get_structure(Some(path.as_str()))
}

async fn general_structure() -> impl IntoResponse {
    get_structure(None)
}

fn get_structure(path: Option<&str>) -> Result<Json<Vec<Entry>>, (StatusCode, &'static str)> {
    let path = get_path(path);
    match get_entries(path) {
        Ok(entries) => Ok(Json(entries)),
        Err(_) => Err((StatusCode::NOT_FOUND, "not found")),
    }
}

fn get_path(path: Option<&str>) -> String {
    format!("data/repository/{}", path.unwrap_or(""))
}

#[derive(Serialize)]
pub struct Entry {
    name: String,
    is_file: bool,
}

fn get_entries<P: AsRef<std::path::Path>>(path: P) -> Result<Vec<Entry>> {
    let read_dir = fs::read_dir(path)?;

    Ok(read_dir
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_ok())
        .filter(|entry| entry.file_name().to_str().is_some())
        .map(|entry| Entry {
            is_file: entry.file_type().unwrap().is_file(),
            name: entry.file_name().to_str().unwrap().to_string(),
        })
        .collect())
}
async fn create_dir(Path(path): Path<String>) -> impl IntoResponse {
    let path = format!("data/repository/{}", path);
    match fs::create_dir_all(path) {
        Ok(_) => (StatusCode::OK, ""),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, ""),
    }
}

#[derive(Deserialize)]
pub struct RenameDirPayload {
    new_name: String,
}

async fn rename_entry(
    Path(path): Path<String>,
    Json(payload): Json<RenameDirPayload>,
) -> impl IntoResponse {
    let path = format!("data/repository/{}", path);
    let mut new_path = PathBuf::from(path.clone());
    new_path.set_file_name(payload.new_name);

    if PathBuf::from(path.clone()).exists().not() {
        return StatusCode::NOT_FOUND;
    }

    match fs::rename(path, new_path) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}
async fn delete_file(Path(path): Path<String>) -> impl IntoResponse {
    let path = format!("data/repository/{}", path);

    if PathBuf::from(path.clone()).exists().not() {
        return StatusCode::NOT_FOUND;
    }

    match fs::remove_file(path) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn delete_dir(Path(path): Path<String>) -> impl IntoResponse {
    let path = format!("data/repository/{}", path);

    if PathBuf::from(path.clone()).exists().not() {
        return StatusCode::NOT_FOUND;
    }

    match fs::remove_dir_all(path) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

#[derive(Deserialize)]
pub struct MoveEntryPayload {
    new_path: String,
}
async fn move_entry(
    Path(path): Path<String>,
    Json(payload): Json<MoveEntryPayload>,
) -> impl IntoResponse {
    let path = PathBuf::from(format!("data/repository/{}", path));
    let name = match match path.file_name() {
        None => return StatusCode::INTERNAL_SERVER_ERROR,
        Some(name) => name,
    }
    .to_str()
    {
        None => return StatusCode::INTERNAL_SERVER_ERROR,
        Some(name) => name,
    };
    let new_path = format!("data/repository/{}/{}", payload.new_path, name);

    if path.exists().not() {
        return StatusCode::NOT_FOUND;
    }

    match fs::rename(path, new_path) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn upload_file(
    Path(path): Path<String>,
    mut multipart: Multipart
) -> impl IntoResponse {
    let field = match match multipart.next_field().await {
        Ok(field) => field,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
    } {
        None => return (StatusCode::BAD_REQUEST, "No file present".to_string()),
        Some(field) => field,
    };
    
    let path = format!("data/repository/{}", path);
    
    let data = match field.bytes().await {
        Ok(bytes) => bytes,
        Err(err) => return (StatusCode::INTERNAL_SERVER_ERROR,err.to_string()),
    };
    
    match fs::write(path, data) {
        Ok(_) => (StatusCode::OK, "".to_string()),
        Err(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Failed to write".to_string()),
    }
}
