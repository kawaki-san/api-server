pub mod category;
pub mod session;
pub mod user;

use entity::async_graphql;

use self::category::CategoryQuery;
use self::session::SessionQuery;
use self::user::UserQuery;

// Add your other ones here to create a unified Query object
// e.x. Query(UserQuery, CategoryQuery, OtherOtherQuery)
#[derive(async_graphql::MergedObject, Default)]
pub struct Query(CategoryQuery, SessionQuery, UserQuery);
