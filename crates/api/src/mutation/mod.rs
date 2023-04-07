pub mod account;
pub mod session;
pub mod user;

use entity::async_graphql;

use self::{account::AccountMutation, session::SessionMutation, user::UserMutation};

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(AccountMutation, SessionMutation, UserMutation);
