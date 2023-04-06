use async_graphql::SimpleObject;
use sea_orm::{entity::prelude::*, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "session")]
#[graphql(concrete(name = "Session", params()))]
/// Used to look up the user in the database
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    /// Unique id used as pk
    pub id: String,
    #[sea_orm(column_type = "Text")]
    /// A randomly generated value that s used to get hold of the session
    pub session_token: String,
    #[sea_orm(column_type = "Text")]
    /// Connects the active session to a user in the database
    pub user_id: String,
    /// The absolute date when the session expires. If a session is accessed past its expiry date,
    pub expires: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::user::Entity",
        from = "Column::UserId",
        to = "super::user::Column::Id",
        on_update = "Cascade",
        on_delete = "Cascade"
    )]
    User,
}

impl Related<super::user::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_session_token(id: &str) -> Select<Entity> {
        Self::find().filter(Column::SessionToken.eq(id))
    }

    pub fn delete_by_id(id: &str) -> DeleteMany<Entity> {
        Self::delete_many().filter(Column::Id.eq(id))
    }
}
