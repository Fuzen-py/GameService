use games::slot_machine::{Response, SlotMachine};
use rocket_contrib::Json;

#[get("/<bet>")]
fn slots(bet: u64) -> Json<Response> {
    Json(Response {
        status_code: 200,
        status: Ok(SlotMachine::new(bet)),
    })
}
