use rocket::response::Redirect;
use rocket::{Route, State};
use crate::UrlController;

pub fn routes() -> Vec<Route> {
    return routes![index, get_url];
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/<url>")]
async fn get_url(url: String, url_controller: &State<UrlController>) -> Option<Redirect> {
    let long_url = url_controller.get_long_url(url).await;

    match long_url {
        Ok(url) => { Some(Redirect::to(url)) }
        Err(_) => { None }
    }
}