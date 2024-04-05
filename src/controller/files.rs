use axum::Router;
use tower_http::services::ServeDir;

pub fn files_router() -> Router {
    Router::new()
        .nest_service("/repository", ServeDir::new("data/repository"))
}