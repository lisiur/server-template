use crate::error::ServerError;

pub type ServerResult<T> = Result<T, ServerError>;
