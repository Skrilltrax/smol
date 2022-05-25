use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

pub(crate) struct Database {
    config: DatabaseConfig,
}

pub(crate) struct DatabaseConfig {
    pub(crate) username: String,
    pub(crate) password: String,
    pub(crate) database_host: String,
    pub(crate) database_name: String,
}

impl Database {
    pub(crate) fn new(database_config: DatabaseConfig) -> Self {
        Database { config: database_config }
    }

    pub async fn create_database(&self) -> Result<Pool<Postgres>, Error> {
        let DatabaseConfig { username, password, database_host, database_name } = &self.config;
        let uri = &format!("postgres://{username}:{password}@{database_host}/{database_name}");

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(uri).await?;

        Ok(pool)
    }
}

