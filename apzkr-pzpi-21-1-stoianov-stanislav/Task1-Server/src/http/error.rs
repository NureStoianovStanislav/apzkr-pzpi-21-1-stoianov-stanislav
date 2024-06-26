use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
struct ErrorMessage {
    error: String,
}

impl IntoResponse for crate::Error {
    fn into_response(self) -> Response {
        use crate::Error;
        let code = match self {
            Error::Validation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            Error::AccountExists => StatusCode::CONFLICT,
            Error::LoggedOff | Error::InvalidCredentials => {
                StatusCode::UNAUTHORIZED
            }
            Error::Unauthorized => StatusCode::FORBIDDEN,
            Error::NotFound => StatusCode::NOT_FOUND,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        let message = ErrorMessage {
            error: self.to_string(),
        };
        (code, Json(message)).into_response()
    }
}
