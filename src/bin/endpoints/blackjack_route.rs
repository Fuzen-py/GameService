#![allow(needless_pass_by_value)]
use ConnectionPool;


use diesel::prelude::*;

use rocket::State;
use api::blackjack::{BlackJack, Response, SessionCount};
use rocket_contrib::Json;

#[get("/")]
fn active_sessions(db_pool: State<ConnectionPool>) -> Json<SessionCount> {
    use games_microservice::schema::blackjack::dsl::*;
    let conn = db_pool.clone().get().unwrap();
    Json(match blackjack
        .filter(status.is_null())
        .count()
        .get_result::<i64>(&*conn)
    {
        Ok(session_count) => SessionCount::count(session_count as u64),
        Err(_) => SessionCount::err("Failed to get active sessions"),
    })
}

#[get("/<user>")]
fn user_info(db_pool: State<ConnectionPool>, user: u64) -> Json<Response> {
    Json(match BlackJack::restore(db_pool.clone(), user) {
        Ok(bj) => Response::success(&bj),
        Err(_) => Response::error(501, "User does not exist"),
    })
}

#[post("/<user>/create/<bet>")]
fn create_user(db_pool: State<ConnectionPool>, user: u64, bet: u64) -> Json<Response> {
    Json(match BlackJack::new(user, bet, db_pool.clone()) {
        Some(bj) => Response::success(&bj),
        None => Response::error(
            501,
            "Failed to create, bet must be claimed before recreating a session.",
        ),
    })
}

#[post("/<user>/hit")]
fn player_hit(db_pool: State<ConnectionPool>, user: u64) -> Json<Response> {
    Json(match BlackJack::restore(db_pool.clone(), user) {
        Ok(mut bj) => match bj.player_hit() {
            Ok(_) => Response::success(&bj),
            Err(err) => Response::error(501, err),
        },
        Err(_) => Response::error(501, "User does not exist"),
    })
}

#[post("/<user>/stay")]
fn player_stay(db_pool: State<ConnectionPool>, user: u64) -> Json<Response> {
    Json(match BlackJack::restore(db_pool.clone(), user) {
        Ok(mut bj) => {
            bj.player_stay();
            Response::success(&bj)
        }
        Err(_) => Response::error(501, "User does not exist"),
    })
}
#[post("/<user>/claim")]
fn claim(db_pool: State<ConnectionPool>, user: u64) -> Json<Response> {
    Json(match BlackJack::restore(db_pool.clone(), user) {
        Ok(bj) => match bj.claim() {
            Ok(bj) => Response::success(&bj),
            Err(_) => Response::error(501, "Game is not over yet"),
        },
        Err(_) => Response::error(501, "User does not exist"),
    })
}
