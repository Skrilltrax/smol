#[derive(Eq, PartialEq, Hash)]
pub(crate) struct UrlShortener {
    base_url: String,
}

impl UrlShortener {
    pub fn new(base_url: String) -> UrlShortener { UrlShortener { base_url } }

    fn shorten(url: String) -> String {
        url
    }
}
