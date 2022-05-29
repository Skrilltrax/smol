use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

#[derive(Eq, PartialEq, Hash)]
pub(crate) struct UrlShortener {
    base_url: String,
}

impl UrlShortener {
    const SHORT_URL_LENGTH: usize = 8;

    pub fn new(base_url: String) -> UrlShortener { UrlShortener { base_url } }

    pub fn generate_short_url(&self) -> String {
        let base_url = self.base_url.to_owned();
        let short_string = self.get_short_part();
        let short_url = base_url + short_string.as_str();

        short_url
    }

    fn get_short_part(&self) -> String {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(UrlShortener::SHORT_URL_LENGTH)
            .map(char::from)
            .collect();

        rand_string
    }
}
