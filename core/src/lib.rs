#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[macro_use]
extern crate rocket;
extern crate bcrypt;
extern crate cookie;
extern crate rocket_contrib;

pub mod endpoints;
pub mod models;
pub mod schema;
pub mod utils;

pub fn start_server() {
    rocket_server().launch();
}

pub fn rocket_server() -> rocket::Rocket {
    rocket::ignite().manage(utils::pg_pool::init_pool()).mount(
        "/",
        routes![
            endpoints::account::account_create,
            endpoints::account::account_email_check
        ],
    )
}
