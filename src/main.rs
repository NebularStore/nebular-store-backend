use crate::controller::config::config_router;
use crate::controller::files::files_router;
use crate::controller::theme::theme_router;
use crate::general_config::{TracingLevel, RELOAD_HANDLE};
use crate::state::init_state;
use axum::Router;
use std::net::SocketAddr;
use axum::http::Method;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing::metadata::LevelFilter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::reload::Handle;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, reload, Registry};

mod admin_config;
mod controller;
mod general_config;
mod state;

#[tokio::main]
async fn main() {
    let state = init_state().expect("Failed to create initial state!");

    let level = state
        .read()
        .await
        .general_config
        .logging
        .max_level
        .clone()
        .unwrap_or(TracingLevel::Warn)
        .level();
    let (filter, reload_handle): (_, Handle<LevelFilter, Registry>) = reload::Layer::new(level);
    RELOAD_HANDLE.set(reload_handle).unwrap();

    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::Layer::default())
        .init();

    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);
    
    let app = Router::new()
        .nest("/config", config_router())
        .nest("/theme", theme_router())
        .nest("/files", files_router())
        .layer(
            ServiceBuilder::new()
                .layer(AddExtensionLayer::new(state.clone()))
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        )
        .layer(cors);

    let port = state.read().await.general_config.server.port;
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to create tcp listener");
    info!("Starting server on port {}", port);
    axum::serve(listener, app).await.expect("Failed to serve")
}
