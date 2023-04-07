use async_graphql::{Context, Object, Result};

use entity::{
    account,
    async_graphql::{self, InputObject},
    sea_orm::{ActiveModelTrait, EntityTrait, Set},
};
use tracing::{debug, error, span, Level};

use crate::Database;

#[derive(Default)]
pub struct AccountMutation;

#[derive(InputObject, Debug)]
pub struct AccountInput {
    provider_account_id: String,
    r#type: String,
    user_id: String,
    provider: String,
    scope: Option<String>,
    id_token: Option<String>,
    expires_in: Option<i32>,
    token_type: Option<String>,
    access_token: Option<String>,
    refresh_token: Option<String>,
}
#[Object]
impl AccountMutation {
    pub async fn create_account(
        &self,
        ctx: &Context<'_>,
        input: AccountInput,
    ) -> Result<account::Model> {
        let span = span!(Level::TRACE, "Create Account");
        let _enter = span.enter();
        debug!("{input:?}");

        let db = ctx.data::<Database>()?;
        let id = cuid2::CuidConstructor::new().with_length(25).create_id();
        let account = account::ActiveModel {
            id: Set(id.clone()),
            provider_account_id: Set(input.provider_account_id),
            r#type: Set(input.r#type),
            user_id: Set(input.user_id),
            provider: Set(input.provider),
            scope: Set(input.scope),
            id_token: Set(input.id_token),
            expires_at: Set(input.expires_in),
            token_type: Set(input.token_type),
            access_token: Set(input.access_token),
            refresh_token: Set(input.refresh_token),
            ..Default::default()
        };
        match account.insert(db.get_connection()).await {
            Ok(model) => {
                debug!(uuid = id, "created account");
                Ok(model)
            }
            Err(e) => {
                error!("create account error {e}");
                Err(e.into())
            }
        }
    }
    async fn delete_account(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<account::Model>> {
        let span = span!(Level::TRACE, "Delete Account");
        let _enter = span.enter();
        debug!("{id:?}");
        let db = ctx.data::<Database>()?;

        let account = account::Entity::find_by_id(&id)
            .one(db.get_connection())
            .await?;
        let res = account::Entity::delete_by_id(&id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            debug!(id = id, "account deleted successfully");
            Ok(account)
        } else {
            error!(id = id, "could not delete account");
            Err("could not delete".into())
        }
    }
}
