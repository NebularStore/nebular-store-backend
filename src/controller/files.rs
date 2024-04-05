use axum::extract::{Path, Query};
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Json, Router};
use std::fs;
use std::ops::Not;
use std::path::PathBuf;
use tower_http::services::ServeDir;
use anyhow::Result;
use axum::http::StatusCode;
use serde::{Deserialize, Serialize};

pub fn files_router() -> Router {
    Router::new()
        .route("/repository/rename/*path", post(rename))
        .route("/repository/create/folder/*path", post(create_dir))
        .nest_service("/repository/get/", ServeDir::new("data/repository"))
        .route("/structure/*path", get(specified_structure))
        .route("/structure/", get(general_structure))
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
        Err(_) => Err((StatusCode::NOT_FOUND, "not found"))
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
    new_name: String
}
async fn rename(Path(path): Path<String>, Json(payload): Json<RenameDirPayload>) -> impl IntoResponse {
    let path = format!("data/repository/{}", path);
    let mut new_path = PathBuf::from(path.clone());
    new_path.set_file_name(payload.new_name);
    
    if (PathBuf::from(path.clone()).exists().not()) {
        return StatusCode::NOT_FOUND;
    }
    
    match fs::rename(path, new_path) {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}