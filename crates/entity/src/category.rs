use async_graphql::SimpleObject;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "category")]
#[graphql(concrete(name = "Category", params()))]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub parent_id: i32,
    pub image_url: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ad::Entity")]
    Ad,
}

impl Related<super::ad::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ad.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_parent_id(id: i32) -> Select<Entity> {
        Self::find().filter(Column::ParentId.eq(id))
    }

    pub fn find_by_name(name: &str) -> Select<Entity> {
        Self::find().filter(Column::Name.like(&format!("%{name}%")))
    }

    pub fn find_by_name_exact(name: &str) -> Select<Entity> {
        Self::find().filter(Column::Name.eq(name))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}
