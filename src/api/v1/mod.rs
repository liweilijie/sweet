use axum::routing::get;
use axum::Router;
async fn hello() -> String {
    "Hello, world!".to_string()
}

pub fn apply_routes() -> Router {
    let router = Router::new();
    router.route("/hello", get(hello))
}
