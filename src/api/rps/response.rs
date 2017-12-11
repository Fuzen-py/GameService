#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Game {
    bet: u64,
    gain: i64,
    result: Option<bool>,
    computer: String,
    player: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Error {
    bet: u64,
    msg: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    status_code: u16,
    status: Result<Game, Error>,
}

impl Response {
    /// Creates a response for a win
    pub fn win(bet: u64, player: String, computer: String) -> Self {
        let gain: i64 = (bet as f64 * 0.25) as i64;

        Self {
            status_code: 200,
            status: Ok(Game {
                bet,
                gain,
                result: Some(true),
                player,
                computer,
            }),
        }
    }

    /// Creates a Response for a loss
    pub fn lose(bet: u64, player: String, computer: String) -> Self {
        Self {
            status_code: 200,
            status: Ok(Game {
                bet,
                gain: -(bet as i64),
                result: Some(false),
                player,
                computer,
            }),
        }
    }

    /// Creates a response for a win
    pub fn draw(bet: u64, player: String, computer: String) -> Self {
        Self {
            status_code: 200,
            status: Ok(Game {
                bet,
                gain: 0,
                result: Some(false),
                player,
                computer,
            }),
        }
    }

    /// Creates a response for an error
    pub fn error(bet: u64, msg: String) -> Self {
        Self {
            status_code: 501,
            status: Err(Error { bet, msg }),
        }
    }
}
