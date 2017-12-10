#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive, test)]
#![allow(unknown_lints)]
extern crate diesel;
extern crate games_microservice;

#[allow(unused_imports, useless_attribute)]
#[macro_use]
extern crate serde_derive;


#[cfg(feature = "web")]
extern crate rocket;

#[cfg(feature = "web")]
extern crate rocket_contrib;

use self::games_microservice::{api, establish_connection_pool, ConnectionPool};

mod endpoints;

#[cfg(feature = "web")]
pub fn create_rocket() -> rocket::Rocket {
    endpoints::router(rocket::ignite().manage(establish_connection_pool()))
}

fn main() {
    create_rocket().launch();
}
