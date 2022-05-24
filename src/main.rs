#![feature(proc_macro_hygiene, decl_macro)]

mod url_shortener;
mod routes;
mod models;
mod schema
mod controllers;

#[macro_use]
extern crate rocket;
extern crate dotenv_codegen;

use dotenv_codegen::dotenv;

fn main() {
    let _base_url = dotenv!("SMOL_BASE_URL").to_string();

    let mut rocket = rocket::ignite();
    rocket = routes::mount_routes(rocket);
    rocket.launch();
}