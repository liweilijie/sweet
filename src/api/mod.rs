use axum::routing::get;
use axum::Router;

macro_rules! reject {
    ($e: expr) => {
        crate::error::Error::Custom($e.to_string())
    };
}

macro_rules! reply {
    ($t: tt) => {
        axum::response::Json(serde_json::json!({"code":0, "data": $t}))
    }
}

mod v1;

const API_PREFIX: &str = "/api/v1";
async fn index() -> &'static str {
    "hello sweet."
}

pub fn apply_routes() -> Router {
    let router = Router::new().route("/", get(index));
    router.nest(API_PREFIX, v1::apply_routes())
}
