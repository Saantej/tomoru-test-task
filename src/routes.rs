use axum::{Router, routing::get};
use axum_client_ip::SecureClientIpSource;

use crate::handlers::handler;

pub fn create_router() -> Router {
    Router::new()
        .route("/ping", get(handler))
        .layer(SecureClientIpSource::ConnectInfo.into_extension())
}
