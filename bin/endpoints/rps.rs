use api::rps as rps_game;
use rocket_contrib::Json;

#[allow(needless_pass_by_value)]
#[get("/<weapon>/<bet>")]
fn rps(weapon: String, bet: u64) -> Json<rps_game::Response> {
    Json(rps_game::rps(bet, &weapon))
}
