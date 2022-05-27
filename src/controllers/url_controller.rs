use anyhow::Result;
use sqlx::types::Uuid;
use crate::url_shortener::UrlShortener;
use crate::UrlDao;

struct UrlController<'a> {
    url_shortener: UrlShortener,
    url_dao: UrlDao<'a>,
}

impl<'a> UrlController<'a> {
    pub fn new(url_shortener: UrlShortener, url_dao: UrlDao<'a>) -> Self {
        Self { url_shortener, url_dao }
    }

    pub async fn remove_url(&self, id: Uuid) -> Result<String> {
        let url_result = self.url_dao.remove_url_by_id(id).await?;
        Ok(url_result.long_url)
    }

    pub fn save_url(&self, long_url: String) -> Result<String> {
        let short_url = self.url_shortener.shorten(long_url);
        Ok(short_url)
    }

    pub fn get_long_url(&self, short_url: String) -> Result<String> {
        Ok("".to_string())
    }
}
