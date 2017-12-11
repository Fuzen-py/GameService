use api::blackjack::{BlackJack, GameState};

#[derive(Deserialize, Serialize)]
pub struct Success {
    pub player_id: u64,
    pub player_hand: Vec<String>,
    // Only first card is shown on first turn
    pub dealer_hand: Vec<String>,
    pub player_score: u64,
    // Only first card value is shown on first turn
    pub dealer_score: u64,
    pub bet: u64,
    pub gain: i64,
    pub game_state: Option<bool>,
    pub player_can_hit: bool,
    // This should always be false if state is set
    pub dealer_can_hit: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status_code: u16,
    pub status: Result<Success, String>,
}

impl Response {
    pub fn success(bj: &BlackJack) -> Self {
        let player_id = bj.player_id;
        let (player_score, player_hand) = bj.player.export();

        let (dealer_score, dealer_hand) = if bj.first_turn {
            let first_card = &bj.dealer.cards[0];
            (u64::from(first_card.value), vec![first_card.to_string()])
        } else {
            bj.dealer.export()
        };

        let state: Option<bool> = match bj.status() {
            GameState::InProgress => None,
            GameState::PlayerWon => Some(true),
            GameState::PlayerLost => Some(false),
        };

        Response {
            status_code: 200,
            status: Ok(Success {
                bet: bj.bet,
                gain: bj.gain,
                game_state: state,
                player_can_hit: !bj.player_stay_status,
                dealer_can_hit: !bj.dealer_stay_status,
                player_id,
                player_hand,
                dealer_hand,
                player_score,
                dealer_score,
            }),
        }
    }

    pub fn error(error_code: u16, error_message: &str) -> Self {
        Self {
            status_code: error_code,
            status: Err(error_message.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SessionCount {
    pub status_code: u16,
    pub status: Result<Counter, String>,
}

#[derive(Serialize, Deserialize)]
pub struct Counter {
    pub active_sessions: u64,
}

impl SessionCount {
    pub fn count(active_sessions: u64) -> Self {
        Self {
            status_code: 200,
            status: Ok(Counter { active_sessions }),
        }
    }
    pub fn err(err_msg: &str) -> Self {
        Self {
            status_code: 500,
            status: Err(err_msg.to_owned()),
        }
    }
}
