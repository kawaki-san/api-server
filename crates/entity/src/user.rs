use async_graphql::SimpleObject;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "user")]
#[graphql(concrete(name = "User", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub name: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub email: Option<String>,
    pub email_verified: Option<DateTimeWithTimeZone>,
    #[sea_orm(column_type = "Text", nullable)]
    pub image: Option<String>,
    pub is_admin: bool,
    pub saved_ads: Option<Vec<String>>,
    pub watched_categories: Option<Vec<i32>>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::ad::Entity")]
    Ad,
    #[sea_orm(has_many = "super::account::Entity")]
    Account,
    #[sea_orm(has_many = "super::session::Entity")]
    Session,
}

impl Related<super::ad::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Ad.def()
    }
}

impl Related<super::account::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Account.def()
    }
}

impl Related<super::session::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Session.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_email(id: String) -> Select<Entity> {
        Self::find().filter(Column::Email.eq(id))
    }

    pub fn delete_by_id(id: &str) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}
