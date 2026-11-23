//! Written by `everything-rust app lift`: every route is answered in Rust.

use std::collections::HashMap;

use axum::Router;
use axum::extract::Request;
use axum::response::Response;

pub mod app;

pub fn router() -> Router {
    Router::new().fallback(fallback)
}

async fn fallback(req: Request) -> Response {
    app::handle("*", HashMap::new(), req).await
}
