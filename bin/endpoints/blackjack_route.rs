use games::blackjack::{BlackJack, Response, SessionCount};
use diesel::prelude::*;
use rocket::State;
use rocket_contrib::Json;
use ConnectionPool;

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[get("/")]
fn active_sessions(db_pool: State<ConnectionPool>) -> Json<SessionCount> {
    use games_microservice::schema::blackjack::dsl::*;

    let conn = db_pool.get().unwrap();
    let result = blackjack
        .filter(status.is_null())
        .count()
        .get_result::<i64>(&*conn);

    Json(match result {
        Ok(session_count) => SessionCount::count(session_count as u64),
        Err(_) => SessionCount::err("Failed to get active sessions"),
    })
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[get("/<user>")]
fn user_info(db_pool: State<ConnectionPool>, user: u64) -> Json<Response> {
    Json(match BlackJack::restore(&db_pool, user) {
        Ok(bj) => Response::success(&bj),
        Err(err) => Response::error(&err),
    })
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/<user>/create/<bet>")]
fn create_user(db_pool: State<ConnectionPool>, user: u64, bet: u64) -> Json<Response> {
    Json(match BlackJack::new(user, bet, db_pool.clone()) {
        Ok(bj) => Response::success(&bj),
        Err(err) => Response::error(&err),
    })
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/<user>/hit")]
fn player_hit(db_pool: State<ConnectionPool>, user: u64) -> Json<Response> {
    Json(match BlackJack::restore(&db_pool, user) {
        Ok(mut bj) => match bj.player_hit() {
            Ok(_) => Response::success(&bj),
            Err(err) => Response::error(&err),
        },
        Err(err) => Response::error(&err),
    })
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/<user>/stay")]
fn player_stay(db_pool: State<ConnectionPool>, user: u64) -> Json<Response> {
    Json(match BlackJack::restore(&db_pool, user) {
        Ok(mut bj) => {
            // TODO: check if this is an error and don't return success if so
            let _ = bj.player_stay();

            Response::success(&bj)
        }
        Err(err) => Response::error(&err),
    })
}

#[cfg_attr(feature = "cargo-clippy", allow(needless_pass_by_value))]
#[post("/<user>/claim")]
fn claim(db_pool: State<ConnectionPool>, user: u64) -> Json<Response> {
    Json(match BlackJack::restore(&db_pool, user) {
        Ok(mut bj) => match bj.claim() {
            Ok(_) => Response::success(&bj),
            Err(err) => Response::error(&err),
        },
        Err(err) => Response::error(&err),
    })
}
