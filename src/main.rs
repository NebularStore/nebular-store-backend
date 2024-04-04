use crate::state::{init_state, SharedState};
use axum::routing::get;
use axum::{Extension, Router};
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::add_extension::AddExtensionLayer;

mod general_config;
mod state;

#[tokio::main]
async fn main() {
    let state = init_state().expect("Failed to create initial state!");
    let app = Router::new().route("/", get(hello_world_with_port)).layer(
        ServiceBuilder::new()
            .layer(AddExtensionLayer::new(state.clone()))
            .into_inner(),
    );

    let port = state.read().await.general_config().server().port();
    let address = SocketAddr::from(([0, 0, 0, 0], port));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Failed to create tcp listener");
    axum::serve(listener, app).await.expect("Failed to serve")
}

async fn hello_world_with_port(Extension(state): Extension<SharedState>) -> String {
    let port = state.read().await.general_config().server().port();
    format!("Hello world from port {}!", port)
}
