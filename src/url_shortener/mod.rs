use rand::{thread_rng, Rng};
use rand::distributions::Alphanumeric;

pub(crate) struct UrlShortener;

impl UrlShortener {
    const SHORT_URL_LENGTH: usize = 8;

    pub fn generate_short_url() -> String {
        Self::get_short_part()
    }

    fn get_short_part() -> String {
        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(UrlShortener::SHORT_URL_LENGTH)
            .map(char::from)
            .collect();

        rand_string
    }
}
