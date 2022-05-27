use sqlx::types::Uuid;

#[derive(Debug)]
pub(crate) struct Url {
    pub(crate) id: Uuid,
    pub(crate) short_url: String,
    pub(crate) long_url: String,
}