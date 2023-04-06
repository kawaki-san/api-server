pub mod user;

use entity::async_graphql;

use self::user::UserMutation;

#[derive(async_graphql::MergedObject, Default)]
pub struct Mutation(UserMutation);
