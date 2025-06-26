use entity::auth_tokens;
use sea_orm::prelude::*;

use crate::{result::AppResult, services::auth_token::AuthTokenService};

impl AuthTokenService {
    pub async fn delete_auth_token_by_token(&self, token: &str) -> AppResult<()> {
        auth_tokens::Entity::delete_many()
            .filter(auth_tokens::Column::Token.eq(token))
            .exec(&self.conn)
            .await?;

        Ok(())
    }

    pub async fn delete_auth_token_by_id(&self, id: Uuid) -> AppResult<()> {
        auth_tokens::Entity::delete_by_id(id).exec(&self.conn).await?;

        Ok(())
    }

    pub async fn delete_auth_token_by_ref_id(&self, ref_id: Uuid) -> AppResult<()> {
        auth_tokens::Entity::delete_many()
            .filter(auth_tokens::Column::RefId.eq(ref_id))
            .exec(&self.conn)
            .await?;

        Ok(())
    }
}
