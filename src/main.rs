//noinspection RsMainFunctionNotFound
#![feature(proc_macro_hygiene, decl_macro)]

mod url_shortener;
mod routes;
mod models;
mod controllers;
mod database;

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use std::env;

use database::{Database, DatabaseConfig};

#[rocket::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_config = DatabaseConfig {
        username: env::var("DATABASE_USER").unwrap(),
        password: env::var("DATABASE_PASSWORD").unwrap(),
        database_host: env::var("DATABASE_HOST").unwrap(),
        database_name: env::var("DATABASE_NAME").unwrap()
    };
    let database = Database::new(database_config);
    let pool = database.create_database().await?;

    let mut rocket = rocket::build();
    rocket = routes::mount_routes(rocket);
    let _ = rocket.manage(pool).launch().await?;

    Ok(())
}