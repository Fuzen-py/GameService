use rocket::Rocket;
mod blackjack_route;
mod slot_machine;
mod coin_toss;
mod rps;

pub fn router(rocket: Rocket) -> Rocket {
    rocket
        .mount(
            "/blackjack",
            routes![
                blackjack_route::active_sessions,
                blackjack_route::user_info,
                blackjack_route::create_user,
                blackjack_route::player_hit,
                blackjack_route::player_stay,
                blackjack_route::claim
            ],
        )
        .mount("/slot_machine", routes![slot_machine::slots])
        .mount("/coin_toss", routes![coin_toss::coin_toss])
        .mount("/rps", routes![rps::rps])
}
#[cfg(test)]
mod test;
