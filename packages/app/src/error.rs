use std::fmt::Display;

use sea_orm::DbErr;
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Exception(#[from] AppException),

    #[error("database error: {0}")]
    Db(#[from] DbErr),

    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
pub enum AppException {
    RoleNotFound,
    UserNotFound,
    GroupNotFound,
    GroupCircleDetected,
}

impl Display for AppException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).unwrap();
        f.write_fmt(format_args!("{}", json.trim_matches('"')))
    }
}

impl std::error::Error for AppException {}
