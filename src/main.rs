#![feature(proc_macro_hygiene, decl_macro)]

mod url_shortener;
mod routes;
mod models;
mod controllers;
mod database;
mod dao;

#[macro_use]
extern crate rocket;
extern crate dotenv;

use dotenv::dotenv;
use std::env;
use std::error::Error;
use sqlx::PgPool;

use crate::database::Database;
use crate::dao::url_dao::UrlDao;


#[rocket::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let _base_url = env::var("SMOL_BASE_URL").expect("You must set the SMOL_BASE_URL environment var!");
    let database_url = env::var("DATABASE_URL").expect("You must set the DATABASE_URL environment var!");

    let pool = initialize_database().await?;
    let url_dao = UrlDao::new(&pool);

    let mut rocket = rocket::build();
    rocket = routes::mount_routes(rocket);
    // let _ = rocket.manage(pool).launch().await?;

    Ok(())
}

async fn initialize_database() -> Result<PgPool, Box<dyn Error>> {
    let database_url = env::var("DATABASE_URL").expect("You must set the DATABASE_URL environment var!");
    let database = Database::new(database_url);
    let pool = database.create_database().await?;
    Ok(pool)
}

async fn initialize_dao(database_url: String) -> Result<PgPool, Box<dyn Error>> {
    let database_url = database_url;
    let database = Database::new(database_url);
    let pool = database.create_database().await?;
    Ok(pool)
}