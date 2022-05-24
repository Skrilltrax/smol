use rocket::Route;

pub fn routes() -> Vec<Route> {
    return routes![index];
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/<url>")]
fn get_url(url: String) -> String {
    // TODO: get the long url from database
    url
}