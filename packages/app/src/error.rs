use std::fmt::Display;

use sea_orm::{DbErr, SqlxError};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("{0}")]
    Exception(#[from] AppException),

    #[error("database error: {0}")]
    Db(#[from] DbErr),

    #[error("sqlx error: {0}")]
    Sqlx(#[from] SqlxError),

    #[error("{0}")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
pub enum AppException {
    RoleNotFound,
    UserNotFound,
    AuthenticationFailed,
    InvalidCredentials,
    DepartmentNotFound,
    UserGroupNotFound,
    RoleGroupNotFound,
    PermissionNotFound,
    PermissionGroupNotFound,
    GroupCircleDetected,
    DepartmentCircleDetected,
}

impl Display for AppException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let json = serde_json::to_string(self).unwrap();
        f.write_fmt(format_args!("{}", json.trim_matches('"')))
    }
}

impl std::error::Error for AppException {}
