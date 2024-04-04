use crate::general_config::TracingLevel;
use crate::state::{init_state, SharedState};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Json, Router};
use std::net::SocketAddr;
use axum::extract::Query;
use serde::Deserialize;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

mod general_config;
mod state;
mod admin_config;

#[tokio::main]
async fn main() {
    let state = init_state().expect("Failed to create initial state!");

    let level = state
        .read()
        .await
        .general_config()
        .logging()
        .max_level()
        .clone()
        .unwrap_or(TracingLevel::Warn)
        .level();
    tracing_subscriber::fmt().with_max_level(level).init();
    let app = Router::new()
        .route("/general_config", get(general_config))
        .route("/admin_config", get(admin_config))
        .route("/check_admin_hash", get(check_admin_hash))
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(state.clone()))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let port = state.read().await.general_config().server().port();
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to create tcp listener");
    info!("Starting server on port {}", port);
    axum::serve(listener, app).await.expect("Failed to serve")
}

async fn general_config(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    Json(state.read().await.general_config().clone())
}

async fn admin_config(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    Json(state.read().await.admin_config().clone())
}

#[derive(Deserialize)]
pub struct AdminCheckParams {
    admin_hash: String
}
async fn check_admin_hash(Extension(state): Extension<SharedState>, admin_hash: Query<AdminCheckParams>) -> impl IntoResponse {
    state.read().await.admin_config().check_admin_hash(admin_hash.0.admin_hash).to_string()
}