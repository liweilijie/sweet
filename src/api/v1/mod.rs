mod auth;
mod medicinal;
mod user;

use crate::middleware::{Cors, JWT};
use axum::Router;
use std::collections::HashMap;
use tower::layer::layer_fn;
use tower_http::auth::RequireAuthorizationLayer;

pub fn apply_routes() -> Router {
    let mut unless = HashMap::new();
    unless.insert(r"^/public".to_string(), "get|post".to_string());
    unless.insert(r"/register".to_string(), "post".to_string());
    unless.insert(r"/login".to_string(), "post".to_string());

    let restrict_layer = RequireAuthorizationLayer::custom(JWT::new(unless));

    auth::apply_routes()
        .merge(user::apply_routes())
        .merge(medicinal::apply_routes())
        .layer(restrict_layer)
        .layer(layer_fn(|inner| Cors { inner }))
}
