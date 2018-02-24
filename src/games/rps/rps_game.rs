use super::{Response, Weapons};

/// Quick RPS Game
/// Weapons: rock/paper/scissors
pub fn rps(bet: u64, weapon: &str) -> Response {
    let weapon = match weapon.parse::<Weapons>() {
        Ok(v) => v,
        Err(_) => {
            return Response::error(bet, String::from("Valid choices are rock/paper/scissors"));
        }
    };

    let comp = Weapons::rand_weapon();

    if weapon == comp {
        Response::draw(bet, weapon.to_string(), comp.to_string())
    } else if weapon > comp {
        // Win
        Response::win(bet, weapon.to_string(), comp.to_string())
    } else {
        Response::lose(bet, weapon.to_string(), comp.to_string())
    }
}
