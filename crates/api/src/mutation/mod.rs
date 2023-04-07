pub mod account;
pub mod user;

use entity::async_graphql;

use self::{account::AccountMutation, user::UserMutation};

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(AccountMutation, UserMutation);
