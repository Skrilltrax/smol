use anyhow::Result;
use sqlx::types::Uuid;
use crate::models::url::Url;
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

    pub async fn save_url(&self, long_url: String) -> Result<String> {
        let short_url = self.url_shortener.generate_short_url();
        let return_url = short_url.to_owned();
        let url = Url { id: Default::default(), short_url, long_url };
        self.url_dao.add_url(url).await?;

        Ok(return_url)
    }

    pub async fn get_long_url(&self, short_url: String) -> Result<String> {
        let long_url = self.url_dao.find_url(short_url).await?;

        Ok(long_url)
    }
}
