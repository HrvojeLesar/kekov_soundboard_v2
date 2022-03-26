use actix_web::{ResponseError, http::StatusCode, HttpResponse};
use oauth2::{basic::BasicErrorResponseType, StandardErrorResponse, RevocationErrorResponseType};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KekServerError {
    #[error(transparent)]
    SendRequestError(#[from] awc::error::SendRequestError),
    #[error(transparent)]
    PayloadError(#[from] actix_web::error::PayloadError),
    #[error(transparent)]
    RequestTokenError(#[from] Box<oauth2::RequestTokenError<KekServerError, StandardErrorResponse<BasicErrorResponseType>>>),
    #[error(transparent)]
    RevocationRequestTokenError(#[from] Box<oauth2::RequestTokenError<KekServerError, StandardErrorResponse<RevocationErrorResponseType>>>),
    #[error(transparent)]
    CookieParseError(#[from] actix_web::cookie::ParseError),
    #[error(transparent)]
    UuidError(#[from] uuid::Error),
    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),
    #[error(transparent)]
    SerdeJsonParseError(#[from] serde_json::Error),
    #[error(transparent)]
    OAuthConfigurationError(#[from] oauth2::ConfigurationError),
    #[error("Recieved request has no session cookie set")]
    SessionCookieNotSet,
    #[error("Invalid Authorization Credentials")]
    InvalidCredentialsError,
    #[error("Enviroment Error")]
    EnvError(#[from] dotenv::Error),
    #[error("Error while parsing JSON: {0}")]
    JsonParseError(#[from] awc::error::JsonPayloadError),
    #[error("Request extensions error")]
    RequestExtensionsError,
    #[error("Canceled authorization error")]
    CanceledAuthError,
    #[error("Time to authorize expired")]
    AuthorizationTimeExpiredError,
    #[error("{0}")]
    Other(String),
}

#[derive(Serialize, Deserialize, Debug)]
struct ApiError<'a> {
    error: &'a str,
    description: &'a str,
}

impl ResponseError for KekServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            KekServerError::InvalidCredentialsError => {
                StatusCode::UNAUTHORIZED
            },
            KekServerError::EnvError(..) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        return HttpResponse::build(self.status_code()).json(
            ApiError {
                error: match self {
                    _ => "Catch all error",
                },
                description: &self.to_string(),
            },
        );
    }

}

pub struct ErrorHelpers{}

impl ErrorHelpers {
    pub fn e500<E: std::fmt::Debug + std::fmt::Display + 'static>(err: E) -> actix_web::Error {
        log::warn!("Session Service encounted an error: {}", err);
        return actix_web::error::ErrorInternalServerError(err);
    }
}

