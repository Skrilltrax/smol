use rocket::Route;

#[post("/", data = "<url>")]
fn shorten_post(url: String) -> String {
    url
}

#[get("/<url>")]
fn shorten_get_path(url: String) -> String {
    url
}

pub fn routes() -> Vec<Route> {
    return routes![shorten_post, shorten_get_path];
}