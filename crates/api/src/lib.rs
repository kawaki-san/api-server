mod mutation;
mod query;

use entity::{
    async_graphql::{EmptySubscription, Schema},
    sea_orm,
};

use self::{mutation::Mutation, query::Query};

pub struct Database {
    connection: sea_orm::DatabaseConnection,
}

impl Database {
    pub async fn new() -> Self {
        let connection = sea_orm::Database::connect(
            std::env::var("DATABASE_URL").expect("DATABASE_URL env var was not set"),
        )
        .await
        .expect("could not connect to database");
        Self { connection }
    }

    pub fn get_connection(&self) -> &sea_orm::DatabaseConnection {
        &self.connection
    }
}

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    let db = Database::new().await;

    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .data(db)
        .finish()
}
