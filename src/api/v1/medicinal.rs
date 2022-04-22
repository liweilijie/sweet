use crate::error;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::Value;

async fn all() -> error::Result<Json<Value>> {
    return Err(reject!("Not implemented"));
}

pub fn apply_routes() -> Router {
    let router = Router::new();

    router.route("/medicinal", get(all))
    // .route("/medicinal", get(all).post(create))
    // .route("/medicinal/:id", get(one).put(update))
    // .route("/medicinal/del/:id", get(delete))
}
