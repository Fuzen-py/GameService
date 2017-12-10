use api::coin_toss::{guess_side, CoinTossResponse};
use rocket_contrib::Json;

#[allow(needless_pass_by_value)]
#[get("/<guess>/<bet>")]
fn coin_toss(guess: String, bet: u64) -> Json<CoinTossResponse> {
    Json(guess_side(bet, &guess))
}
