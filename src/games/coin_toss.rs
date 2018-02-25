use rand::{thread_rng, Rng};

const WEIGHT: u32 = 2;

#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, PartialOrd, Ord, Serialize)]
pub enum Coin {
    Heads,
    Tails,
}

impl Coin {
    /// Flips a coun, returning a Coin with its current Side up
    pub fn flip() -> Coin {
        let mut rng = thread_rng();
        if rng.gen_weighted_bool(WEIGHT) {
            Coin::Heads
        } else {
            Coin::Tails
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoinTossResult {
    player: Coin,
    computer: Coin,
    bet: u64,
    gain: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CoinTossResponse {
    status_code: u16,
    status: Result<CoinTossResult, String>,
}

impl CoinTossResponse {
    pub fn win(bet: u64, side: Coin, guess: Coin) -> Self {
        Self {
            status_code: 200,
            status: Ok(CoinTossResult {
                player: guess,
                computer: side,
                bet,
                gain: (bet / 2) as i64,
            }),
        }
    }

    pub fn lose(bet: u64, side: Coin, guess: Coin) -> Self {
        Self {
            status_code: 200,
            status: Ok(CoinTossResult {
                player: guess,
                computer: side,
                bet,
                gain: -(bet as i64),
            }),
        }
    }

    pub fn err(err: String) -> Self {
        Self {
            status_code: 501,
            status: Err(err),
        }
    }
}

/// Guess a coin side
pub fn guess_side(bet: u64, side: &str) -> CoinTossResponse {
    let side_lowercase = side.to_lowercase();

    if !["heads", "h", "tails", "t"].contains(&&side_lowercase[..]) {
        return CoinTossResponse::err(String::from("Not a valid side, heads/tails."));
    };

    let guessed_side = if side.starts_with('h') {
        Coin::Heads
    } else {
        Coin::Tails
    };

    let side = Coin::flip();

    if guessed_side == side {
        CoinTossResponse::win(bet, side, guessed_side)
    } else {
        CoinTossResponse::lose(bet, side, guessed_side)
    }
}
