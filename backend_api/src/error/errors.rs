use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use log::error;
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
    #[error(transparent)]
    TokioOneshotRecvError(#[from] tokio::sync::oneshot::error::RecvError),
    #[error(transparent)]
    ElapsedError(#[from] tokio::time::error::Elapsed),
    #[error(transparent)]
    WsBotClientError(#[from] crate::ws::ws_server::ClientError),
    #[error(transparent)]
    ToStrError(#[from] actix_http::header::ToStrError),
    #[error(transparent)]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error(transparent)]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error(transparent)]
    SerdeUrlencodedError(#[from] serde_urlencoded::ser::Error),
    #[error("Provided files faild to upload")]
    NoFilesUploadedError,
    #[error("Invalid Authorization Credentials")]
    InvalidCredentialsError,
    #[error("Discord request error")]
    DiscordRequestError,
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
    #[error("Authorized user not found error")]
    AuthorizedUserNotFoundError,
    #[error("User not in cache error")]
    UserNotInCacheError,
    #[error("Time to authorize expired")]
    AuthorizationTimeExpiredError,
    #[error("Invalid guild id error")]
    InvalidGuildIdError,
    #[error("Guild file does not exist")]
    GuildFileDoesNotExistError,
    #[error("Invalid file id error")]
    InvalidFileIdError,
    #[error("Unauthorized file access: {0}")]
    UnauthorizedFileAccessError(String),
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
            KekServerError::SendRequestError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::PayloadError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::RequestTokenError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::RevocationRequestTokenError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::CookieParseError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::UuidError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::SqlxError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::SerdeJsonParseError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::OAuthConfigurationError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::MultipartError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::BlockingError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::IOError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::WsClientError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::ActixWebError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::ActixMailboxError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::TokioOneshotRecvError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::ElapsedError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::WsBotClientError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::ToStrError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::ParseFloatError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::ParseIntError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::ReqwestError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::SerdeUrlencodedError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::NoFilesUploadedError => StatusCode::BAD_REQUEST,
            KekServerError::InvalidCredentialsError => StatusCode::UNAUTHORIZED,
            KekServerError::DiscordRequestError => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::NotInGuildError => StatusCode::UNAUTHORIZED,
            KekServerError::UnableToGetMimeError => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::WrongMimeTypeError => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::FileTooLargeError => StatusCode::BAD_REQUEST,
            KekServerError::EnvError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::JsonParseError(..) => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::RequestExtensionsError => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::AuthorizedUserNotFoundError => StatusCode::INTERNAL_SERVER_ERROR,
            KekServerError::UserNotInCacheError => StatusCode::UNAUTHORIZED,
            KekServerError::AuthorizationTimeExpiredError => StatusCode::BAD_REQUEST,
            KekServerError::InvalidGuildIdError => StatusCode::NOT_FOUND,
            KekServerError::GuildFileDoesNotExistError => StatusCode::NOT_FOUND,
            KekServerError::InvalidFileIdError => StatusCode::NOT_FOUND,
            KekServerError::UnauthorizedFileAccessError(..) => StatusCode::UNAUTHORIZED,
            KekServerError::Other(..) => StatusCode::BAD_REQUEST,
        }
    }

    fn error_response(&self) -> HttpResponse {
        return HttpResponse::build(self.status_code()).json(ApiError {
            error: match self {
                KekServerError::SendRequestError(..) => "awc_send_request_error",
                KekServerError::PayloadError(..) => "payload_error",
                KekServerError::RequestTokenError(..) => "request_token_error",
                KekServerError::RevocationRequestTokenError(..) => "revocation_request_token_error",
                KekServerError::CookieParseError(..) => "cookie_parse_error",
                KekServerError::UuidError(..) => "uuid_error",
                KekServerError::SqlxError(..) => "sqlx_error",
                KekServerError::SerdeJsonParseError(..) => "serde_json_parse_error",
                KekServerError::OAuthConfigurationError(..) => "oauth_config_error",
                KekServerError::MultipartError(..) => "multipart_error",
                KekServerError::BlockingError(..) => "blocking_error",
                KekServerError::IOError(..) => "io_error",
                KekServerError::WsClientError(..) => "ws_client_error",
                KekServerError::ActixWebError(..) => "actix_web_error",
                KekServerError::ActixMailboxError(..) => "actix_mailbox_error",
                KekServerError::TokioOneshotRecvError(..) => "tokio_oneshot_recv_error",
                KekServerError::ElapsedError(..) => "elapsed_error",
                KekServerError::WsBotClientError(..) => "ws_bot_client_error",
                KekServerError::ToStrError(..) => "to_str_error",
                KekServerError::SerdeUrlencodedError(..) => "serde_urlencoded_error",
                KekServerError::ParseFloatError(..) => "parse_float_error",
                KekServerError::ParseIntError(..) => "parse_int_error",
                KekServerError::ReqwestError(..) => "reqwest_error",
                KekServerError::NoFilesUploadedError => "no_files_uploaded_error",
                KekServerError::InvalidCredentialsError => "invalid_credentials_error",
                KekServerError::DiscordRequestError => "discord_request_error",
                KekServerError::NotInGuildError => "not_in_guild_error",
                KekServerError::UnableToGetMimeError => "unable_to_get_mime_error",
                KekServerError::WrongMimeTypeError => "wrong_mime_type_error",
                KekServerError::FileTooLargeError => "file_too_large_error",
                KekServerError::EnvError(..) => "enviroment_error",
                KekServerError::JsonParseError(..) => "json_parse_error",
                KekServerError::RequestExtensionsError => "request_extension_error",
                KekServerError::AuthorizedUserNotFoundError => "user_not_found_error",
                KekServerError::UserNotInCacheError => "user_not_in_cache_error",
                KekServerError::AuthorizationTimeExpiredError => "authorization_time_expired_error",
                KekServerError::InvalidGuildIdError => "invalid_guild_id_error",
                KekServerError::GuildFileDoesNotExistError => "guild_file_does_not_exist_error",
                KekServerError::InvalidFileIdError => "invalid_file_id_error",
                KekServerError::UnauthorizedFileAccessError(..) => "unauthorized_file_access_error",
                KekServerError::Other(..) => "other",
            },
            description: &self.to_string(),
        });
    }
}
