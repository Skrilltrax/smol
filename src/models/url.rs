use sqlx::types::Uuid;

pub struct Url {
    id: Uuid,
    short_url: String,
    long_url: String,
}