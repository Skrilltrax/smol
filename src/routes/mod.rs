use rocket::Rocket;

pub mod index;
pub mod shorten;

pub fn mount_routes(rocket: Rocket) -> Rocket {
    rocket
        .mount("/", index::routes())
        .mount("/api/shorten", shorten::routes())
}