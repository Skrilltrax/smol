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

use anyhow::Result;
use dotenv::dotenv;
use std::env;
use sqlx::PgPool;
use controllers::url_controller::UrlController;
use::rocket_db_pools::Database as RocketDatabase;

use crate::database::{Database, Smol};
use crate::dao::url_dao::UrlDao;
use crate::url_shortener::UrlShortener;


#[rocket::main]
async fn main() -> Result<()> {
    dotenv().ok();
    env_logger::init();

    let base_url = env::var("SMOL_BASE_URL").expect("You must set the SMOL_BASE_URL environment var!");
    let database_url = env::var("DATABASE_URL").expect("You must set the DATABASE_URL environment var!");

    let url_shortener = UrlShortener::new(base_url);
    let pool = initialize_database(database_url).await?;
    let url_dao = UrlDao::new(pool);
    let url_controller = UrlController::new(url_shortener, url_dao);

    let _ = routes::mount_routes(rocket::build()).attach(Smol::init()).manage(url_controller).launch().await?;

    Ok(())
}

async fn initialize_database(database_url: String) -> Result<PgPool> {
    let database = Database::new(database_url);
    let pool = database.create_database().await?;
    Ok(pool)
}