use diesel::Queryable;
use diesel::sql_types::Uuid;

#[derive(Queryable)]
pub struct Url {
    id: Uuid,
    short_url: String,
    long_url: String,
}