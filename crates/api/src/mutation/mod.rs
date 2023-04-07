pub mod account;
pub mod session;
pub mod user;
pub mod verification_token;

use entity::async_graphql;

use self::{
    account::AccountMutation, session::SessionMutation, user::UserMutation,
    verification_token::VerificationTokenMutation,
};

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(
    AccountMutation,
    SessionMutation,
    UserMutation,
    VerificationTokenMutation,
);
