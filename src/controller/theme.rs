use axum::body::Body;
use axum::http::{Request, Response, StatusCode, Uri};
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{Extension, Router};
use tower::ServiceExt;
use tower_http::services::fs::ServeFileSystemResponseBody;
use tower_http::services::ServeFile;

use crate::state::SharedState;

pub fn theme_router() -> Router {
    Router::new()
        .route("/company_name", get(company_name))
        .route("/icon", get(icon))
}

async fn company_name(Extension(state): Extension<SharedState>) -> impl IntoResponse {
    state.read().await.general_config.theme.company_name.clone()
}

async fn icon(
    Extension(state): Extension<SharedState>,
    uri: Uri,
) -> Result<Response<ServeFileSystemResponseBody>, (StatusCode, String)> {
    let req = Request::builder().uri(uri).body(Body::empty()).unwrap();

    let path = state.read().await.general_config.theme.icon_path.clone();
    let response = ServeFile::new(path).oneshot(req).await;
    if response.is_err() {
        return Err((StatusCode::INTERNAL_SERVER_ERROR, String::from("")));
    }
    Ok(response.unwrap())
}
