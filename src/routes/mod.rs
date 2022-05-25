use rocket::{Build, Rocket};

pub mod index;
pub mod shorten;

pub fn mount_routes(rocket: Rocket<Build>) -> Rocket<Build> {
    rocket
        .mount("/", index::routes())
        .mount("/api/shorten", shorten::routes())
}