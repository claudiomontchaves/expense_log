use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ErrorType {
    EntityNotFound,
    InternalServerError,
    InvalidUserOrPassword,
    InvalidToken,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Error {
    pub error_type: ErrorType,
    pub message: String,
}

impl Error {
    pub fn new(error_type: ErrorType, message: String) -> Error {
        Error {
            error_type,
            message,
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let body = serde_json::to_string(&self).unwrap();

        let status = match self.error_type {
            ErrorType::EntityNotFound => StatusCode::NOT_FOUND,
            ErrorType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::InvalidUserOrPassword => StatusCode::UNAUTHORIZED,
            ErrorType::InvalidToken => StatusCode::UNAUTHORIZED,
        };
        (status, body).into_response()
    }
}
