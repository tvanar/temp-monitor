use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum AppError {
    TemplateError,
    InvalidCredentials,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::TemplateError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Template error").into_response()
            }

            AppError::InvalidCredentials => {
                (StatusCode::UNAUTHORIZED, "Invalid Credentials").into_response()
            }
        }
    }
}
