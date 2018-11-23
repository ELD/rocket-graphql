#![feature(decl_macro, proc_macro_hygiene)]

extern crate rocket;

use rocket::{get, routes};

fn main() {
    rocket::ignite()
        .mount("/", routes![graphql_handler])
        .launch();
}

#[get("/")]
fn graphql_handler() -> &'static str {
    "Static endpoint!"
}
