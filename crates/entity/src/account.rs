use async_graphql::SimpleObject;
use sea_orm::{entity::prelude::*, Condition};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, SimpleObject)]
#[sea_orm(table_name = "account")]
#[graphql(concrete(name = "Account", params()))]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
    pub id: String,
    #[sea_orm(column_type = "Text")]
    pub user_id: String,
    #[sea_orm(column_type = "Text")]
    pub r#type: String,
    #[sea_orm(column_type = "Text")]
    pub provider: String,
    #[sea_orm(column_type = "Text")]
    pub provider_account_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub refresh_token: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub access_token: Option<String>,
    pub expires_at: Option<i32>,
    #[sea_orm(column_type = "Text", nullable)]
    pub token_type: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub scope: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub id_token: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub session_state: Option<String>,
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
    pub fn find_by_access_token(access_token: &str) -> Select<Entity> {
        Self::find().filter(Column::AccessToken.eq(access_token))
    }
    pub fn find_by_provider_and_id(provider: &str, provider_account_id: &str) -> Select<Entity> {
        Self::find().filter(
            Condition::all()
                .add(Column::ProviderAccountId.eq(provider_account_id))
                .add(Column::Provider.eq(provider)),
        )
    }
    pub fn find_by_user_id(user_id: &str) -> Select<Entity> {
        Self::find().filter(Column::UserId.eq(user_id))
    }
}
