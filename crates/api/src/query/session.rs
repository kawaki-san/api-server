use async_graphql::{Context, Object, Result};
use entity::{
    async_graphql::{self, SimpleObject},
    session, user,
};
use tracing::{debug, span, Level};

use crate::Database;

#[derive(Default)]
pub struct SessionQuery;

#[derive(SimpleObject)]
pub struct SessionedUser {
    session: session::Model,
    user: user::Model,
}

#[Object]
impl SessionQuery {
    async fn get_user_by_session_token(
        &self,
        ctx: &Context<'_>,
        token: String,
    ) -> Result<Option<SessionedUser>> {
        let span = span!(Level::TRACE, "get_user_by_session_token");
        let _enter = span.enter();
        debug!("get user by session");
        let db = ctx.data::<Database>().unwrap();
        let entity = session::Entity::find_by_session_token(&token)
            .find_also_related(user::Entity)
            .one(db.get_connection())
            .await
            .map_err(|e| {
                eprintln!("{e}");
                e.to_string()
            })?
            .map(|(session, user)| user.map(|user| SessionedUser { session, user }));
        if let Some(out) = entity {
            Ok(out)
        } else {
            Ok(None)
        }
    }
}
