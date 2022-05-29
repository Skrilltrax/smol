use anyhow::Result;
use sqlx::{PgPool, query, query_as};
use sqlx::types::Uuid;
use crate::models::url::Url;

pub(crate) struct UrlDao {
    pool: PgPool,
}

impl UrlDao {
    pub(crate) fn new(pool: PgPool) -> UrlDao {
        UrlDao { pool }
    }

    pub(crate) async fn get_all_urls(&self) -> Result<Vec<Url>> {
        let result: Vec<Url> = query_as!(Url, "SELECT * FROM urls").fetch_all(&self.pool).await?;
        Ok(result)
    }

    pub(crate) async fn add_url(&self, url: Url) -> Result<()> {
        query_as!(Url, "INSERT INTO urls(slug, long_url) VALUES($1, $2)", url.slug, url.long_url).fetch_all(&self.pool).await?;
        Ok(())
    }

    pub(crate) async fn remove_url_by_id(&self, id: Uuid) -> Result<Url> {
        let find_result = query_as!(Url, "SELECT * FROM urls WHERE id = $1", id).fetch_one(&self.pool).await?;
        query!("DELETE FROM urls WHERE id = $1", id).execute(&self.pool).await?;
        Ok(find_result)
    }

    pub(crate) async fn find_url(&self, slug: String) -> Result<String> {
        let find_result = query_as!(Url, "SELECT * FROM urls WHERE slug = $1", slug).fetch_one(&self.pool).await?;
        Ok(find_result.long_url)
    }
}