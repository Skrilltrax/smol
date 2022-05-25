#![feature(proc_macro_hygiene, decl_macro)]

mod url_shortener;
mod routes;
mod models;
mod schema;
mod controllers;

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let _base_url = env::var("SMOL_BASE_URL").unwrap_or_default();

    let mut rocket = rocket::ignite();
    rocket = routes::mount_routes(rocket);
    rocket.launch();
}