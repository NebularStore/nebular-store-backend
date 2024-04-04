use crate::general_config::TracingLevel;
use crate::state::SharedState;
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde::Deserialize;
use std::path::PathBuf;

pub fn config_router() -> Router {
    Router::new()
        .route("/general", get(general_config))
        .route("/admin", get(admin_config))
        .route("/check_admin", get(check_admin_hash))
        .route("/change", post(change_config))
}

async fn general_config(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    Json(state.read().await.general_config.clone())
}

async fn admin_config(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    Json(state.read().await.admin_config.clone())
}

#[derive(Deserialize)]
pub struct AdminCheckParams {
    admin_hash: String,
}
async fn check_admin_hash(
    Extension(state): Extension<SharedState>,
    admin_hash: Query<AdminCheckParams>,
) -> impl IntoResponse {
    state
        .read()
        .await
        .admin_config
        .check_admin_hash(admin_hash.0.admin_hash)
        .to_string()
}

#[derive(Deserialize)]
pub struct ChangeConfigPayload {
    path: String,
    value: String,
}
async fn change_config(
    Extension(state): Extension<SharedState>,
    Json(payload): Json<ChangeConfigPayload>,
) -> impl IntoResponse {
    let mut path = payload.path.split('.');
    let category = match path.next() {
        None => return (StatusCode::BAD_REQUEST, "No category provided"),
        Some(category) => category.to_lowercase(),
    };
    let part = match path.next() {
        None => return (StatusCode::BAD_REQUEST, "No part provided"),
        Some(part) => part.to_lowercase(),
    };
    let param = match path.next() {
        None => return (StatusCode::BAD_REQUEST, "No param provided"),
        Some(param) => param.to_lowercase(),
    };

    let worked = match category.as_str() {
        "admin" => match part.as_str() {
            "credentials" => match param.as_str() {
                "password" => {
                    state
                        .write()
                        .await
                        .admin_config
                        .credentials
                        .set_password(payload.value);
                    true
                }
                _ => false,
            },
            _ => false,
        },
        "general" => match part.as_str() {
            "server" => match param.as_str() {
                "port" => return (StatusCode::BAD_REQUEST, "Cannot change port via api"),
                _ => false,
            },
            "theme" => match param.as_str() {
                "icon_path" => {
                    state.write().await.general_config.theme.icon_path =
                        PathBuf::from(payload.value);
                    true
                }
                "company_name" => {
                    state.write().await.general_config.theme.company_name = payload.value;
                    true
                }
                _ => false,
            },
            "logging" => match param.as_str() {
                "max_level" => {
                    println!("{}", payload.value);
                    let tracing_level = match TracingLevel::from_str(payload.value.as_str()) {
                        Ok(level) => level,
                        _ => {
                            return (
                                StatusCode::BAD_REQUEST,
                                "Invalid log level provided (Trace, Debug, Info, Warn, Error)",
                            )
                        }
                    };
                    state
                        .write()
                        .await
                        .general_config
                        .logging
                        .set_max_level(tracing_level);
                    true
                }
                _ => false,
            },
            _ => false,
        },
        _ => false,
    };

    if worked {
        if state.read().await.admin_config.write().is_err() {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save config");
        }
        if state.read().await.general_config.write().is_err() {
            return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to save config");
        }
        (StatusCode::OK, "")
    } else {
        (StatusCode::BAD_REQUEST, "Unknown config")
    }
}
