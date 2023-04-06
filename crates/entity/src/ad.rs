use async_graphql::SimpleObject;
use sea_orm::{entity::prelude::*, Condition, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "ad")]
#[graphql(concrete(name = "Ad", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: String,
    pub title: String,
    pub price: Decimal,
    pub user_id: String,
    pub images: Option<Vec<String>>,
    pub created_at: DateTimeWithTimeZone,
    pub deleted_at: Option<DateTimeWithTimeZone>,
    pub is_live: bool,
    pub viewed_by: Vec<String>,
    pub description: String,
    pub category_id: i32,
    pub show_user_email: bool,
    pub show_user_mobile: bool,
    pub region_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::category::Entity",
        from = "Column::CategoryId",
        to = "super::category::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Category,
    #[sea_orm(
        belongs_to = "super::region::Entity",
        from = "Column::RegionId",
        to = "super::region::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Region,
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    User,
}

impl Related<super::category::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Category.def()
    }
}

impl Related<super::region::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Region.def()
    }
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id(id: &str) -> Select<Entity> {
        Self::find().filter(Column::Id.eq(id))
    }

    pub fn find_by_category(id: i32) -> Select<Entity> {
        Self::find().filter(Column::CategoryId.eq(id))
    }

    pub fn find_by_region(id: i32) -> Select<Entity> {
        Self::find().filter(Column::RegionId.eq(id))
    }

    pub fn find_by_live_status(status: bool) -> Select<Entity> {
        Self::find().filter(Column::IsLive.eq(status))
    }

    pub fn find_price_at_most(price: Decimal) -> Select<Entity> {
        Self::find().filter(Condition::any().add(Column::Price.lte(price)))
    }

    pub fn find_price_at_least(price: Decimal) -> Select<Entity> {
        Self::find().filter(Condition::any().add(Column::Price.gte(price)))
    }

    pub fn find_by_title(title: &str) -> Select<Entity> {
        Self::find().filter(
            Condition::all()
                .add(Column::Title.like(&format!("%{title}%")))
                .add(Column::IsLive.eq(true)),
        )
    }

    pub fn find_by_title_no_exact(title: &str) -> Select<Entity> {
        Self::find().filter(
            Condition::all()
                .add(Condition::any().add(Column::Title.like(&format!("%{title}%"))))
                .add(Column::IsLive.eq(true)),
        )
    }

    pub fn find_by_user_id(id: &str) -> Select<Entity> {
        Self::find().filter(Column::UserId.eq(id))
    }

    pub fn delete_by_id(id: i32) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}
