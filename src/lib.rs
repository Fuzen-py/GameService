#![recursion_limit = "128"]
#![feature(custom_derive, test, const_fn, custom_attribute)]

#[macro_use]
extern crate cfg_if;
#[macro_use(c)]
extern crate cute;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use(Serialize, Deserialize)]
extern crate serde_derive;

extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate rand;
extern crate regex;

pub use diesel::prelude::*;
pub use diesel::pg::PgConnection;

use dotenv::dotenv;
use r2d2::Pool;
use r2d2_diesel::ConnectionManager;
use std::env;

pub mod api;
pub mod models;
pub mod schema;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

embed_migrations!("migrations");

cfg_if! {
    if #[cfg(test)] {
        fn db_url() -> String {
            env::var("GAMESERVICE_TEST_DATABASE_URL")
                .expect("GAMESERVICE_DATABASE_URL must be set")
        }
    } else {
        fn db_url() -> String {
            env::var("GAMESERVICE_TEST_DATABASE_URL")
                .expect("GAMESERVICE_DATABASE_URL must be set")
        }
    }
}

pub fn establish_connection_pool() -> ConnectionPool {
    dotenv().expect("Error loading env variables");

    let manager = ConnectionManager::<PgConnection>::new(db_url());

    let pool = match env::var("MAX_POOL") {
        Ok(size) => r2d2::Pool::builder()
            .max_size(size.parse::<u32>().expect("MAX_POOL is not a u32"))
            .build(manager),
        Err(_) => r2d2::Pool::new(manager),
    }.expect("Failed to create connection pool");

    // Run migrations
    match pool.get() {
        Ok(conn) => {
            embedded_migrations::run(&*conn).expect("Error running migrations");
        }
        Err(why) => {
            error!("Error obtaining conn to run migrations: {:?}", why);
        }
    }

    #[cfg(test)]
    {
        use schema::blackjack::dsl::*;
        let conn = pool.get().unwrap();
        let _num = diesel::delete(blackjack.filter(id.is_not_null()))
            .execute(&*conn)
            .expect("Error deleting Previous BlackJack Test data");
    }

    pool
}
