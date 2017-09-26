use rand::{thread_rng, Rng};

const WEIGHT: u32 = 2;

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Coin {
    Heads,
    Tails,
}

impl Coin {
    pub fn flip() -> Coin {
        let mut rng = thread_rng();
        if rng.gen_weighted_bool(WEIGHT) {
            Coin::Heads
        } else {
            Coin::Tails
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CoinTossResult {
    player: Coin,
    computer: Coin,
    bet: u64,
    gain: i64,
}

#[derive(Serialize, Deserialize)]
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
                gain: -1 * bet as i64,
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

pub fn guess_side(bet: u64, side: &str) -> CoinTossResponse {
    let valid: bool = ["heads", "h", "tails", "t"]
        .iter()
        .any(|&i| i == side.to_lowercase());
    if !valid {
        return CoinTossResponse::err(String::from("Not a valid side, heads/tails."));
    };
    let guessed_side: Coin = if side.starts_with('h') {
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

#[cfg(test)]
mod test {
    extern crate test;
    use self::test::Bencher;
    use super::guess_side;
    #[bench]
    fn bench_coin(bench: &mut Bencher) {
        bench.iter(|| guess_side(0, "h"))
    }
}
