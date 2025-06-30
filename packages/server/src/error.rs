use std::fmt::Display;

use app::error::{AppError, AppException};
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use sea_orm::{DbErr, SqlxError};
use strum::Display;
use thiserror::Error;

use crate::{response::ResponseErrorJson, result::ServerResult};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("ServerException::{0}")]
    Exception(#[from] ServerException),

    #[error("AppError::{0}")]
    App(#[from] AppError),

    #[error("IOError::{0}")]
    IO(#[from] std::io::Error),

    #[error("MultipartError::{0}")]
    Multipart(#[from] axum::extract::multipart::MultipartError),

    #[error("UUIDError::{0}")]
    Uuid(#[from] uuid::Error),

    #[error("DatabaseError::{0}")]
    Db(#[from] DbErr),

    #[error("SqlxError::{0}")]
    Sqlx(#[from] SqlxError),

    #[error("ConfigError::{0}")]
    Config(#[from] config::ConfigError),

    #[error("Anyhow::{0}")]
    Anyhow(#[from] anyhow::Error),
}

impl ServerError {
    pub fn status(&self) -> StatusCode {
        match &self {
            &Self::Exception(exception) => match exception.code {
                ServerExceptionCode::Unauthorized => StatusCode::UNAUTHORIZED,
                ServerExceptionCode::Forbidden => StatusCode::FORBIDDEN,
                ServerExceptionCode::NotFound => StatusCode::NOT_FOUND,
                ServerExceptionCode::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            },
            &Self::App(err) => match err {
                &AppError::Exception(ref exception) => match exception {
                    &AppException::AuthenticationFailed => StatusCode::UNAUTHORIZED,
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                },
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn code(&self) -> String {
        match &self {
            &Self::Exception(exception) => exception.code.to_string(),
            _ => self.status().as_str().to_string(),
        }
    }
}

impl From<&str> for ServerError {
    fn from(value: &str) -> Self {
        Self::Anyhow(anyhow::anyhow!(value.to_string()))
    }
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        let error_data = ResponseErrorJson::new(self.code(), self.to_string());
        (self.status(), serde_json::to_string(&error_data).unwrap()).into_response()
    }
}

#[derive(Debug, Display)]
#[strum(serialize_all = "snake_case")]
pub enum ServerExceptionCode {
    Unauthorized,
    Forbidden,
    InternalServerError,
    NotFound,
}

impl From<ServerExceptionCode> for ServerError {
    fn from(value: ServerExceptionCode) -> Self {
        Self::Exception(ServerException::new(value))
    }
}

impl From<(ServerExceptionCode, &str)> for ServerError {
    fn from(value: (ServerExceptionCode, &str)) -> Self {
        Self::Exception(ServerException::new(value.0).message(value.1))
    }
}

#[derive(Debug)]
pub struct ServerException {
    code: ServerExceptionCode,
    message: Option<String>,
}

impl Display for ServerException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.code))
    }
}

impl std::error::Error for ServerException {}

impl<T> From<ServerException> for ServerResult<T> {
    fn from(value: ServerException) -> Self {
        Err(value.into())
    }
}

impl ServerException {
    pub fn new(code: ServerExceptionCode) -> Self {
        Self {
            code,
            message: None,
        }
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }
}
