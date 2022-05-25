use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub(crate) struct Database {
    url: String,
}

impl Database {
    pub(crate) fn new(database_url: String) -> Self {
        Database { url: database_url }
    }

    pub async fn create_database(&self) -> Result<Pool<Postgres>, Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.url).await?;

        Ok(pool)
    }
}

