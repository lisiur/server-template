use sea_orm::DbErr;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("0")]
    Db(#[from] DbErr),
}
