use crate::{
    models::auth_token::AuthToken, result::AppResult, services::auth_token::AuthTokenService,
};
use entity::auth_tokens;
use sea_orm::prelude::*;

impl AuthTokenService {
    pub async fn query_auth_token_by_id(&self, id: Uuid) -> AppResult<Option<AuthToken>> {
        let auth_token = auth_tokens::Entity::find_by_id(id).one(&self.conn).await?;

        Ok(auth_token.map(Into::into))
    }

    pub async fn query_auth_tokens_by_ref_id(&self, ref_id: Uuid) -> AppResult<Vec<AuthToken>> {
        let auth_token = auth_tokens::Entity::find()
            .filter(auth_tokens::Column::RefId.eq(ref_id))
            .all(&self.conn)
            .await?;

        Ok(auth_token.into_iter().map(Into::into).collect())
    }
}
