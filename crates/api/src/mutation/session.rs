use async_graphql::{Context, Object, Result};
use entity::{
    async_graphql::{self, InputObject},
    sea_orm::{prelude::DateTimeWithTimeZone, ActiveModelTrait, Set, Unchanged},
    session,
};
use tracing::{debug, error, span, Level};

use crate::Database;

#[derive(Default)]
pub struct SessionMutation;

#[derive(InputObject, Debug)]
pub struct SessionInput {
    session_token: String,
    user_id: String,
    expires: DateTimeWithTimeZone,
}

#[Object]
impl SessionMutation {
    async fn create_session(
        &self,
        ctx: &Context<'_>,
        input: SessionInput,
    ) -> Result<session::Model> {
        let span = span!(Level::TRACE, "Create Session");
        let _enter = span.enter();
        debug!("input={input:?}");
        let db = ctx.data::<Database>()?;
        let id = cuid2::CuidConstructor::new().with_length(25).create_id();
        let session = session::ActiveModel {
            id: Set(id.clone()),
            expires: Set(input.expires),
            user_id: Set(input.user_id),
            session_token: Set(input.session_token),
        };

        debug!(id = id, "session created successfully");

        Ok(session.insert(db.get_connection()).await?)
    }

    pub async fn update_session(
        &self,
        ctx: &Context<'_>,
        session_token: String,
        user_id: Option<String>,
        expires: Option<DateTimeWithTimeZone>,
    ) -> Result<session::Model> {
        let span = span!(Level::TRACE, "Update Session");
        let _enter = span.enter();
        debug!(
            token = session_token,
            "user_id={user_id:?} expires={expires:?}"
        );
        let db = ctx.data::<Database>()?;

        if let Ok(Some(token)) = session::Entity::find_by_session_token(&session_token)
            .one(db.get_connection())
            .await
        {
            let session = session::ActiveModel {
                id: Unchanged(token.id),
                session_token: Unchanged(token.session_token),
                user_id: match user_id {
                    Some(user_id) => Set(user_id),
                    None => Unchanged(token.user_id),
                },
                expires: match expires {
                    Some(expires) => Set(expires),
                    None => Unchanged(token.expires),
                },
            }
            .update(db.get_connection())
            .await?;
            debug!(token = session_token, "updated {session:?}");
            Ok(session)
        } else {
            error!(token = session_token, "session not found");
            Err("no session was found".into())
        }
    }

    pub async fn delete_session(
        &self,
        ctx: &Context<'_>,
        session_token: String,
    ) -> Result<session::Model> {
        let span = span!(Level::TRACE, "Delete Session");
        let _enter = span.enter();
        debug!(token = session_token, "input");
        let db = ctx.data::<Database>()?;

        if let Some(session) = session::Entity::find_by_session_token(&session_token)
            .one(db.get_connection())
            .await?
        {
            let res = session::Entity::delete_by_id(&session_token)
                .exec(db.get_connection())
                .await?;

            if res.rows_affected <= 1 {
                debug!(token = session_token, "deleted {session:?}");
                Ok(session)
            } else {
                error!(token = session_token, "could not delete");
                Err("could not delete".into())
            }
        } else {
            error!(token = session_token, "could not delete");
            Err("could not delete".into())
        }
    }
}
