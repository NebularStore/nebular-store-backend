use crate::general_config::TracingLevel;
use crate::state::{init_state, SharedState};
use axum::routing::get;
use axum::{Extension, Router};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::trace::TraceLayer;
use tracing::info;

mod general_config;
mod state;

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

    let app = Router::new().route("/", get(hello_world_with_port)).layer(
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

async fn hello_world_with_port(Extension(state): Extension<SharedState>) -> String {
    let port = state.read().await.general_config().server().port();
    format!("Hello world from port {}!", port)
}
