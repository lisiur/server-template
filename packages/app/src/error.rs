use std::fmt::Display;

use sea_orm::{DbErr, SqlxError};
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("AppException::{0}")]
    Exception(#[from] AppException),

    #[error("IOError::{0}")]
    IO(#[from] std::io::Error),

    #[error("DatabaseError::{0}")]
    Db(#[from] DbErr),

    #[error("SqlxError::{0}")]
    Sqlx(#[from] SqlxError),

    #[error("Anyhow::{0}")]
    Anyhow(#[from] anyhow::Error),
}

#[derive(Debug, Serialize)]
pub enum AppException {
    RoleNotFound,
    UserNotFound,
    AlreadyExists,
    AuthenticationFailed,
    InvalidCredentials,
    UploadNotFound,
    UploadChunkIncomplete,
    FileHashMismatch,
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
