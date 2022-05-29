use anyhow::Result;
use rocket::{Route, State};
use rocket::form::Form;
use crate::models::url::RequestUrl;
use crate::UrlController;

#[post("/", data = "<url>")]
async fn shorten_post(url: Form<RequestUrl>, url_controller: &State<UrlController>) -> String {
    let long_url = url.url.to_owned();

    let result = url_controller.save_url(long_url).await;
    match result {
        Ok(url) => { url }
        Err(_) => { "Cannot save URL".to_string() }
    }
}

pub fn routes() -> Vec<Route> {
    return routes![shorten_post];
}