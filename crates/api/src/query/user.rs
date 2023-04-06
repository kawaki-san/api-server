use async_graphql::{Context, Object, Result};
use entity::{async_graphql, sea_orm::EntityTrait, user};
use tracing::{debug, span, Level};

use crate::Database;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn get_user_by_id(&self, ctx: &Context<'_>, id: String) -> Result<Option<user::Model>> {
        let span = span!(Level::TRACE, "get_user_by_id");
        let _enter = span.enter();
        debug!("get single user");
        let db = ctx.data::<Database>().unwrap();

        Ok(user::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user_by_email(
        &self,
        ctx: &Context<'_>,
        email: String,
    ) -> Result<Option<user::Model>> {
        let span = span!(Level::TRACE, "get_user_by_email");
        let _enter = span.enter();
        debug!("get single user");
        let db = ctx.data::<Database>().unwrap();

        Ok(user::Entity::find_by_email(email)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_user_by_provider(
        &self,
        ctx: &Context<'_>,
        id: Option<String>,
        name: Option<String>,
    ) -> Result<Option<user::Model>> {
        let span = span!(Level::TRACE, "get_user_by_provider");
        let _enter = span.enter();
        debug!("get single user by provider");
        let _db = ctx.data::<Database>().unwrap();
        if id.is_none() && name.is_none() {
            return Err("need at least id or name".into());
        }
        todo!("find by provider information")
    }
}
