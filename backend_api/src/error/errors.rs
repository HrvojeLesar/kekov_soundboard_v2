use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use oauth2::{basic::BasicErrorResponseType, RevocationErrorResponseType, StandardErrorResponse};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum KekServerError {
    #[error(transparent)]
    SendRequestError(#[from] awc::error::SendRequestError),
    #[error(transparent)]
    PayloadError(#[from] actix_web::error::PayloadError),
    #[error(transparent)]
    RequestTokenError(
        #[from]
        Box<
            oauth2::RequestTokenError<
                KekServerError,
                StandardErrorResponse<BasicErrorResponseType>,
            >,
        >,
    ),
    #[error(transparent)]
    RevocationRequestTokenError(
        #[from]
        Box<
            oauth2::RequestTokenError<
                KekServerError,
                StandardErrorResponse<RevocationErrorResponseType>,
            >,
        >,
    ),
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
    #[error(transparent)]
    MultipartError(#[from] actix_multipart::MultipartError),
    #[error(transparent)]
    BlockingError(#[from] actix_web::error::BlockingError),
    #[error(transparent)]
    IOError(#[from] std::io::Error),
    #[error(transparent)]
    WsClientError(#[from] awc::error::WsClientError),
    #[error(transparent)]
    ActixWebError(#[from] actix_web::Error),
    #[error(transparent)]
    ActixMailboxError(#[from] actix::MailboxError),
    #[error("Provided files faild to upload")]
    NoFilesUploadedError,
    #[error("Recieved request has no session cookie set")]
    SessionCookieNotSet,
    #[error("Invalid Authorization Credentials")]
    InvalidCredentialsError,
    #[error("Discord request error")]
    DiscordRequestError,
    #[error("Disallowed mime type")]
    DisallowedMimeTypeError,
    #[error("Not in guild error")]
    NotInGuildError,
    #[error("Unable to get mime from file")]
    UnableToGetMimeError,
    #[error("Wrong mime type")]
    WrongMimeTypeError,
    #[error("File too large")]
    FileTooLargeError,
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
    #[error("Invalid guild id error")]
    InvalidGuildIdError,
    #[error("Guild file does not exist")]
    GuildFileDoesNotExistError,
    #[error("Invalid file id error")]
    InvalidFileIdError,
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
            KekServerError::InvalidCredentialsError => StatusCode::UNAUTHORIZED,
            KekServerError::EnvError(..) => StatusCode::UNAUTHORIZED,
            _ => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> HttpResponse {
        return HttpResponse::build(self.status_code()).json(ApiError {
            error: match self {
                _ => "Catch all error",
            },
            description: &self.to_string(),
        });
    }
}

pub struct ErrorHelpers {}

impl ErrorHelpers {
    pub fn e500<E: std::fmt::Debug + std::fmt::Display + 'static>(err: E) -> actix_web::Error {
        log::warn!("Session Service encounted an error: {}", err);
        return actix_web::error::ErrorInternalServerError(err);
    }
}
