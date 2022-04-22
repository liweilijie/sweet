use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use bcrypt::BcryptError;
use config::ConfigError;
use serde_json::json;
use validator::ValidationErrors;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, thiserror::Error)]
#[error("...")]
pub enum Error {
    #[error("{0}")]
    ConfigFailed(#[from] ConfigError),

    #[error("{0}")]
    HashPassword(#[from] BcryptError),
    #[error("{0}")]
    Custom(String),

    #[error("{0}")]
    Jsonwebtoken(#[from] jsonwebtoken::errors::Error),

    #[error("{0}")]
    DbError(#[from] rbatis::core::Error),

    #[error("{0}")]
    Validate(#[from] ValidationErrors),
}

impl Error {
    fn get_codes(&self) -> (StatusCode, u16) {
        match *self {
            // 4XX Errors
            Error::ConfigFailed(_) => (StatusCode::BAD_REQUEST, 40001),
            Error::Validate(_) => (StatusCode::BAD_REQUEST, 40002),

            Error::DbError(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50001),
            Error::HashPassword(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50009),
            Error::Custom(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50010),
            Error::Jsonwebtoken(_) => (StatusCode::INTERNAL_SERVER_ERROR, 50011),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status_code, code) = self.get_codes();
        let message = self.to_string();
        let body = Json(json!({ "code": code, "message": message }));

        (status_code, body).into_response()
    }
}
