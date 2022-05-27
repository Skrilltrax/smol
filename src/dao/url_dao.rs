use std::error::Error;
use sqlx::{PgPool, query, query_as};
use sqlx::types::Uuid;
use crate::models::url::Url;

pub(crate) struct UrlDao<'a> {
    pool: &'a PgPool,
}

impl<'a> UrlDao<'a> {
    pub(crate) fn new(pool: &'a PgPool) -> UrlDao {
        UrlDao { pool }
    }

    pub(crate) async fn get_all_urls(&self) -> Result<Vec<Url>, Box<dyn Error>> {
        let result: Vec<Url> = query_as!(Url, "SELECT * FROM urls").fetch_all(self.pool).await?;
        Ok(result)
    }

    pub(crate) async fn add_url(&self, url: Url) -> Result<(), Box<dyn Error>> {
        query_as!(Url, "INSERT INTO urls(short_url, long_url) VALUES($1, $2)", url.short_url, url.long_url).fetch_all(self.pool).await?;
        Ok(())
    }

    pub(crate) async fn remove_url_by_id(&self, id: Uuid) -> Result<Url, Box<dyn Error>> {
        let find_result = query_as!(Url, "SELECT * FROM urls WHERE id = $1", id).fetch_one(self.pool).await?;
        let removed = query!("DELETE FROM urls WHERE id = $1", id).execute(self.pool).await?;
        return Ok(find_result)
    }
}