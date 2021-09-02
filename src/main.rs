#[macro_use]
extern crate rocket_okapi;
#[cfg(test)] mod tests;

mod db;
mod models;
mod requests;
mod responses;
mod route;

use rocket::*;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};

#[openapi]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

pub fn build_rocket() -> Rocket<Build> {
    rocket::build()
    .mount(
        "/",
        routes_with_openapi![
            index,
        ],
    )
    .mount(
        "/swagger-ui/",
        make_swagger_ui(&SwaggerUIConfig {
            url: "../openapi.json".to_owned(),
            ..Default::default()
        }),
    )
}

#[launch]
fn rocket() -> _ {
    build_rocket()
}
