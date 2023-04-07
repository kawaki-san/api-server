use async_graphql::{Context, Object, Result};
use entity::{account, async_graphql, sea_orm::EntityTrait, user};
use tracing::{debug, span, Level};

use crate::Database;

#[derive(Default, Debug)]
pub struct AccountQuery;

#[Object]
impl AccountQuery {
    async fn get_account_by_id(
        &self,
        ctx: &Context<'_>,
        id: String,
    ) -> Result<Option<account::Model>> {
        let span = span!(Level::TRACE, "get_account_by_id()");
        let _enter = span.enter();
        debug!("getting account by id");
        let db = ctx.data::<Database>().unwrap();

        Ok(account::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user_by_account_id(
        &self,
        ctx: &Context<'_>,
        provider: String,
        provider_account_id: String,
    ) -> Result<Option<user::Model>> {
        let span = span!(Level::TRACE, "get_user_by_account_id()");
        let _enter = span.enter();
        debug!("getting user by account id");
        let db = ctx.data::<Database>().unwrap();
        let entity = account::Entity::find_by_provider_and_id(&provider, &provider_account_id)
            .find_also_related(user::Entity)
            .one(db.get_connection())
            .await
            .map_err(|e| {
                eprintln!("{e}");
                e.to_string()
            })?
            .map(|(_, user)| user);
        match entity {
            Some(entity) => Ok(entity),
            None => Ok(None),
        }
    }

    async fn get_account_by_user_id(
        &self,
        ctx: &Context<'_>,
        user_id: String,
    ) -> Result<Option<account::Model>> {
        let span = span!(Level::TRACE, "get_account_by_user_id()");
        let _enter = span.enter();
        debug!("getting account by user id");
        let db = ctx.data::<Database>().unwrap();
        let entity = account::Entity::find_by_user_id(&user_id)
            .one(db.get_connection())
            .await
            .map_err(|e| {
                eprintln!("{e}");
                e.to_string()
            })?;
        match entity {
            Some(entity) => Ok(Some(entity)),
            None => Ok(None),
        }
    }
}
