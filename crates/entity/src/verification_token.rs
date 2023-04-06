use async_graphql::SimpleObject;
use sea_orm::{entity::prelude::*, Condition, DeleteMany};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "verification_token")]
#[graphql(concrete(name = "VerificationToken", params()))]
pub struct Model {
    #[sea_orm(primary_key, column_type = "Text", auto_increment = false)]
    pub identifier: String,
    #[sea_orm(column_type = "Text")]
    pub token: String,
    pub expires: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl Entity {
    pub fn find_by_id_and_token(identifier: &str, token: &str) -> Select<Entity> {
        Self::find().filter(
            Condition::all()
                .add(Column::Token.eq(token))
                .add(Column::Identifier.eq(identifier)),
        )
    }

    pub fn delete_by_id_and_token(identifier: &str, token: &str) -> DeleteMany<Entity> {
        Self::delete_many().filter(
            Condition::all()
                .add(Column::Token.eq(token))
                .add(Column::Identifier.eq(identifier)),
        )
    }
}
