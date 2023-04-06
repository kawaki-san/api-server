use entity::async_graphql;

use self::user::UserQuery;

pub mod user;

// Add your other ones here to create a unified Query object
// e.x. Query(UserQuery, CategoryQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(UserQuery);
