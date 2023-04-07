use async_graphql::{Context, Object, Result};

use entity::{
    async_graphql::{self, InputObject},
    sea_orm::{prelude::DateTimeWithTimeZone, ActiveModelTrait, Set},
    verification_token,
};
use tracing::{debug, error, span, Level};

use crate::Database;

#[derive(Default)]
pub struct VerificationTokenMutation;

#[derive(InputObject, Debug)]
pub struct VerificationTokenInput {
    identifier: String,
    token: String,
    expires: DateTimeWithTimeZone,
}

#[Object]
impl VerificationTokenMutation {
    async fn create_verification_token(
        &self,
        ctx: &Context<'_>,
        input: VerificationTokenInput,
    ) -> Result<verification_token::Model> {
        let span = span!(Level::TRACE, "Create Verification Token");
        let _enter = span.enter();
        debug!("{input:?}");
        let db = ctx.data::<Database>()?;

        let verification_token = verification_token::ActiveModel {
            identifier: Set(input.identifier),
            token: Set(input.token),
            expires: Set(input.expires),
        };
        Ok(verification_token.insert(db.get_connection()).await?)
    }

    async fn delete_verification_token(
        &self,
        ctx: &Context<'_>,
        identifier: String,
        token: String,
    ) -> Result<verification_token::Model> {
        let span = span!(Level::TRACE, "Delete Verification Token");
        let _enter = span.enter();
        debug!("{identifier:?}, {token:?}");
        let db = ctx.data::<Database>()?;

        if let Some(model) = verification_token::Entity::find_by_id_and_token(&identifier, &token)
            .one(db.get_connection())
            .await?
        {
            let res = verification_token::Entity::delete_by_id_and_token(&identifier, &token)
                .exec(db.get_connection())
                .await?;

            if res.rows_affected <= 1 {
                debug!("{identifier:?}, {token:?} deleted successfully");
                Ok(model)
            } else {
                error!("{identifier:?}, {token:?} deletion error");
                Err("could not delete".into())
            }
        } else {
            error!("{identifier:?}, {token:?} deletion error");
            Err("could not delete".into())
        }
    }
}
