use axum::http::StatusCode;
use tower::BoxError;

pub async fn handle_error(e: BoxError) -> (StatusCode, String) {
    if e.is::<tower::timeout::error::Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "request timeout".to_string())
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "unhandled error".to_string(),
        )
    }
}
