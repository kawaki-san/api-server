use async_graphql::{Context, Object, Result};
use entity::{async_graphql, category, sea_orm::EntityTrait};
use tracing::{debug, span, Level};

use crate::Database;

#[derive(Default)]
pub struct CategoryQuery;

#[Object]
impl CategoryQuery {
    async fn get_categories(&self, ctx: &Context<'_>) -> Result<Vec<category::Model>> {
        let span = span!(Level::TRACE, "get_categories");
        let _enter = span.enter();
        debug!("getting all categories");
        let db = ctx.data::<Database>().unwrap();

        Ok(category::Entity::find()
            .all(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_category_by_id(
        &self,
        ctx: &Context<'_>,
        id: i32,
    ) -> Result<Option<category::Model>> {
        let span = span!(Level::TRACE, "get_category");
        let _enter = span.enter();
        debug!("getting a single category");
        let db = ctx.data::<Database>().unwrap();

        Ok(category::Entity::find_by_id(id)
            .one(db.get_connection())
            .await
            .map_err(|e| e.to_string())?)
    }

    async fn get_sub_categories(
        &self,
        ctx: &Context<'_>,
        parent_id: Option<i32>,
    ) -> Result<Vec<category::Model>> {
        let span = span!(Level::TRACE, "get sub categories");
        let _enter = span.enter();
        debug!("getting sub categories");
        let db = ctx.data::<Database>().unwrap();

        Ok(
            category::Entity::find_by_parent_id(parent_id.unwrap_or_default())
                .all(db.get_connection())
                .await
                .map_err(|e| e.to_string())?,
        )
    }
}
