use anyhow::Result;
use sqlx::{PgPool};
use sqlx::postgres::PgPoolOptions;

#[derive(rocket_db_pools::Database)]
#[database("postgres_smol")]
pub (crate) struct Smol(PgPool);

pub(crate) struct Database {
    url: String,
}

impl Database {
    pub(crate) fn new(database_url: String) -> Self {
        Database { url: database_url }
    }

    pub async fn create_database_connection(&self) -> Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.url).await?;

        Ok(pool)
    }
}

