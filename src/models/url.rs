use sqlx::types::Uuid;

#[derive(Debug)]
pub(crate) struct Url {
    pub(crate) id: Uuid,
    pub(crate) slug: String,
    pub(crate) long_url: String,
}

#[derive(FromForm)]
pub(crate) struct RequestUrl {
    pub(crate) url: String
}
