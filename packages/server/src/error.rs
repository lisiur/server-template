use std::fmt::Display;

use app::error::AppError;
use axum::response::{IntoResponse, Response};
use http::StatusCode;
use sea_orm::DbErr;
use thiserror::Error;

use crate::{rest::RestResponseErrorJson, result::ServerResult};

#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Exception: {0}")]
    Exception(#[from] Exception),

    #[error("App error: {0}")]
    App(#[from] AppError),

    #[error("Database error: {0}")]
    Db(#[from] DbErr),

    #[error("Config error: {0}")]
    Config(#[from] config::ConfigError),

    #[error("Anyhow error: {0}")]
    Anyhow(#[from] anyhow::Error),
}

impl ServerError {
    pub fn status(&self) -> StatusCode {
        match &self {
            &Self::Exception(exception) => exception.status.clone(),
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn code(&self) -> String {
        match &self {
            &Self::Exception(exception) => exception.code.to_string(),
            _ => self.status().to_string(),
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
        let error_data = RestResponseErrorJson::new(self.code(), self.to_string());
        (self.status(), error_data).into_response()
    }
}

#[derive(Debug)]
pub struct Exception {
    status: StatusCode,
    code: String,
    message: Option<String>,
}

impl Display for Exception {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}", self.code))
    }
}

impl std::error::Error for Exception {}

impl<T> From<Exception> for ServerResult<T> {
    fn from(value: Exception) -> Self {
        Err(value.into())
    }
}

impl Exception {
    pub fn new(code: &str) -> Self {
        Self {
            status: StatusCode::OK,
            code: code.to_string(),
            message: None,
        }
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.status = status;
        self
    }

    pub fn message(mut self, message: &str) -> Self {
        self.message = Some(message.to_string());
        self
    }
}
