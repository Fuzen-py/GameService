#![recursion_limit = "128"]
#![feature(custom_derive, test, const_fn, custom_attribute)]

#[macro_use]
extern crate cfg_if;
#[macro_use(c)]
extern crate cute;
#[cfg(feature = "auto_save")]
#[macro_use]
extern crate diesel;
#[cfg(feature = "auto_save")]
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;

#[cfg(feature = "auto_save")]
#[macro_use]
extern crate log;
#[macro_use(Serialize, Deserialize)]
extern crate serde_derive;

extern crate dotenv;
#[cfg(feature = "auto_save")]
extern crate r2d2;
#[cfg(feature = "auto_save")]
extern crate r2d2_diesel;
extern crate rand;
extern crate regex;

#[cfg(feature = "auto_save")]
pub use diesel::prelude::*;
#[cfg(feature = "auto_save")]
pub use diesel::pg::PgConnection;
#[cfg(feature = "auto_save")]
use dotenv::dotenv;
#[cfg(feature = "auto_save")]
use r2d2::Pool;
#[cfg(feature = "auto_save")]
use r2d2_diesel::ConnectionManager;
#[cfg(feature = "auto_save")]
use std::env;

pub mod games;
#[cfg(feature = "auto_save")]
pub mod models;
#[cfg(feature = "auto_save")]
pub mod schema;

#[cfg(feature = "auto_save")]
pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;
#[cfg(feature = "auto_save")]
embed_migrations!("migrations");
cfg_if! {
    if #[cfg(not(feature = "auto_save"))] {
    }
    else if #[cfg(test)] {
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

#[cfg(feature = "auto_save")]
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
