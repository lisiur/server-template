use std::ops::Deref;

use axum::extract::FromRequestParts;
use http::request::Parts;
use sea_orm::DatabaseConnection;

#[derive(Debug)]
pub struct AppService<T>(T);

impl<T> Deref for AppService<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S, T> FromRequestParts<S> for AppService<T>
where
    S: Send + Sync,
    T: From<DatabaseConnection>,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let conn = parts.extensions.get::<DatabaseConnection>().unwrap();

        Ok(AppService(conn.to_owned().into()))
    }
}
