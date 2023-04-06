use async_graphql::{Context, Object, Result};
use entity::{
    async_graphql::{self, InputObject},
    sea_orm::{prelude::DateTimeWithTimeZone, ActiveModelTrait, EntityTrait, Set, Unchanged},
    user,
};
use tracing::{debug, error, span, Level};

use crate::Database;

#[derive(Default)]
pub struct UserMutation;

#[derive(InputObject, Debug)]
pub struct UserInput {
    id: Option<String>,
    name: Option<String>,
    email: Option<String>,
    email_verified: Option<DateTimeWithTimeZone>,
    image: Option<String>,
    is_admin: Option<bool>,
    saved_ads: Option<Vec<String>>,
    watched_categories: Option<Vec<i32>>,
}

#[Object]
impl UserMutation {
    async fn create_user(&self, ctx: &Context<'_>, input: UserInput) -> Result<user::Model> {
        let span = span!(Level::TRACE, "Create User");
        let _enter = span.enter();
        debug!("input={input:?}");
        let db = ctx.data::<Database>()?;
        let uuid_len = if let Ok(Some(user)) = user::Entity::find().one(db.get_connection()).await {
            user.id.len()
        } else {
            25
        } as u16;
        let mut id = cuid2::CuidConstructor::new()
            .with_length(uuid_len)
            .create_id();

        // create new id
        while !user::Entity::find_by_id(&id)
            .all(db.get_connection())
            .await?
            .is_empty()
        {
            // increase the uuid length if we already found a match
            id = cuid2::CuidConstructor::new()
                .with_length(uuid_len + 1)
                .create_id();
        }

        let user = user::ActiveModel {
            name: Set(input.name),
            email: Set(input.email),
            image: Set(input.image),
            email_verified: Set(input.email_verified),
            id: Set(id.clone()),
            is_admin: Set(input.is_admin.unwrap_or(false)),
            watched_categories: Set(input.watched_categories),
            saved_ads: Set(input.saved_ads),
        };
        debug!(id = id, "created successfully");
        Ok(user.insert(db.get_connection()).await?)
    }

    async fn update_user(
        &self,
        ctx: &Context<'_>,
        id: String,
        input: UserInput,
    ) -> Result<user::Model> {
        let span = span!(Level::TRACE, "Update User");
        let _enter = span.enter();
        debug!(id = id, "input={input:?}");
        let db = ctx.data::<Database>()?;

        if let Ok(Some(user)) = user::Entity::find_by_id(&id).one(db.get_connection()).await {
            let user = user::ActiveModel {
                id: Unchanged(user.id),
                email: match input.email {
                    Some(email) => Set(Some(email)),
                    _ => Unchanged(user.email),
                },
                email_verified: match input.email_verified {
                    Some(email_verified) => Set(Some(email_verified)),
                    _ => Unchanged(user.email_verified),
                },
                image: match input.image {
                    Some(image) => Set(Some(image)),
                    _ => Unchanged(user.image),
                },
                is_admin: match input.is_admin {
                    Some(is_admin) => Set(is_admin),
                    _ => Unchanged(user.is_admin),
                },
                name: match input.name {
                    Some(name) => Set(Some(name)),
                    _ => Unchanged(user.name),
                },
                saved_ads: match input.saved_ads {
                    Some(saved_ads) => Set(Some(saved_ads)),
                    _ => Unchanged(user.saved_ads),
                },
                watched_categories: match input.watched_categories {
                    Some(watched_categories) => Set(Some(watched_categories)),
                    _ => Unchanged(user.watched_categories),
                },
            }
            .update(db.get_connection())
            .await?;
            debug!(id = id, "user updated={user:?}");
            Ok(user)
        } else {
            error!(id = id, "user not found");
            Err("user not found".into())
        }
    }

    async fn delete_user(&self, ctx: &Context<'_>, id: String) -> Result<Option<user::Model>> {
        let span = span!(Level::TRACE, "Delete User");
        let _enter = span.enter();
        debug!(id = id, "");
        let db = ctx.data::<Database>()?;

        let user = user::Entity::find_by_id(&id)
            .one(db.get_connection())
            .await?;
        let res = user::Entity::delete_by_id(&id)
            .exec(db.get_connection())
            .await?;

        if res.rows_affected <= 1 {
            debug!(id = id, "user deleted");
            Ok(user)
        } else {
            error!(id = id, "could not delete user");
            Err("could not delete".into())
        }
    }
}
