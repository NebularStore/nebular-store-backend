use crate::general_config::TracingLevel;
use crate::state::{init_state};
use axum::{Router};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;
use tracing::info;
use crate::controller::config::config_router;

mod general_config;
mod state;
mod admin_config;
mod controller;

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
        .nest("/config", config_router())
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