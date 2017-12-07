use ConnectionPool;
use api::blackjack::{Deck, Hand, Session};

use diesel::prelude::*;
use diesel;

#[derive(PartialEq)]
pub enum GameState {
    InProgress,
    PlayerWon,
    PlayerLost,
}


// TODO: Implement Surrender
// TODO: Implement Insurrence

pub struct BlackJack {
    pub player: Hand,
    pub player_id: u64,
    pub dealer: Hand,
    deck: Deck,
    pub bet: u64,
    pub first_turn: bool, // Used for responses
    pub player_stay_status: bool,
    pub dealer_stay_status: bool,
    pub gain: i64,
    db_pool: ConnectionPool,
    claimed: bool,
}


impl BlackJack {
    #[allow(needless_pass_by_value)]
    pub fn new(player_id: u64, new_bet: u64, db_pool: ConnectionPool) -> Option<Self> {
        use schema::blackjack as blackjack_schema;
        use schema::blackjack::dsl::*;
        // TODO: Make this safer (low)
        let conn = db_pool.clone().get().unwrap();
        let num: i64 = blackjack
            .filter(id.eq(player_id as i64))
            .count()
            .get_result(&*conn)
            // TODO: Make this safer
            .unwrap_or_default();
        if num != 0 {
            return None;
        }
        let mut new_deck = Deck::new();
        let mut player = Hand::new();
        let mut dealer = Hand::new();
        player.add_card(new_deck.draw());
        player.add_card(new_deck.draw());
        dealer.add_card(new_deck.draw());
        dealer.add_card(new_deck.draw());
        let num: i64 = blackjack
            .filter(id.eq(player_id as i64))
            .count()
            .get_result(&*conn)
            // TODO: Make this safer
            .unwrap_or_default();
        if num == 0 {
            let sess = Session {
                id: player_id as i64,
                bet: Some(new_bet as i64),
                dealer_hand: dealer.export().1,
                dealer_stay: false,
                deck: new_deck.export(),
                first_turn: true,
                player_hand: player.export().1,
                player_stay: false,
                status: None,
            };
            let _: Session = diesel::insert_into(blackjack_schema::table)
                .values(&sess)
                .get_result(&*conn)
                .expect("Error saving sessions");
        }
        Some(Self {
            player_id: player_id,
            player: player,
            dealer: dealer,
            deck: new_deck,
            bet: new_bet,
            first_turn: true,
            player_stay_status: false,
            dealer_stay_status: false,
            db_pool: db_pool.clone(),
            claimed: false,
            gain: 0i64,
        })
    }

    #[allow(needless_pass_by_value)]
    pub fn restore(db_pool: ConnectionPool, player: u64) -> Result<Self, ()> {
        use schema::blackjack::dsl::*;
        // TODO: Make this safer (low)
        let conn = db_pool.clone().get().unwrap();
        let results = blackjack
            .filter(id.eq(player as i64))
            .limit(1)
            .load::<Session>(&*conn)
            // TODO: make this safer
            .unwrap_or_default();
        if results.len() != 1 {
            return Err(()); // There should be one result if not, nothing is found
        }
        let session: &Session = &results[0];
        if session.bet.is_none() {
            return Err(()); // Game is over, Start a new one
        }
        let player_bet = session.bet.unwrap();

        Ok(Self {
            // TODO: Exceptions are not acceptable in production, handle them
            player: Hand {
                cards: c![card.parse().unwrap(), for card in &session.player_hand],
            },
            player_id: session.id as u64,
            dealer: Hand {
                cards: c![card.parse().unwrap(), for card in &session.dealer_hand],
            },
            deck: Deck {
                cards: c![card.parse().unwrap(), for card in &session.deck],
            },
            bet: player_bet as u64,
            player_stay_status: session.player_stay,
            dealer_stay_status: session.dealer_stay,
            first_turn: session.first_turn,
            db_pool: db_pool.clone(),
            claimed: false,
            gain: 0i64,
        })
    }

    pub fn player_hit(&mut self) -> Result<(), &'static str> {
        match self.status() {
            GameState::InProgress => if !self.player_stay_status {
                self.first_turn = false;
                Ok(self.player.add_card(self.deck.draw()))
            } else {
                Err("You already pressed stay")
            },
            GameState::PlayerLost => Err("You already lost"),
            GameState::PlayerWon => Err("You already won"),
        }
    }

    pub fn player_stay(&mut self) {
        if !self.player_stay_status {
            self.player_stay_status = true;
            self.dealer_play().unwrap();
        }
    }

    fn dealer_hit(&mut self) -> Result<(), &'static str> {
        match self.status() {
            GameState::InProgress => if !self.dealer_stay_status {
                Ok(self.dealer.add_card(self.deck.draw()))
            } else {
                Err("The Dealer already pressed stay")
            },
            GameState::PlayerWon => Err("The dealer already lost"),
            GameState::PlayerLost => Err("The dealer already won"),
        }
    }

    fn dealer_stay(&mut self) {
        self.dealer_stay_status = true
    }

    pub fn status(&self) -> GameState {
        let player_score = self.player.score();
        let dealer_score = self.dealer.score();
        if self.player.cards.len() == 5 {
            return GameState::PlayerWon;
        };
        if self.dealer.cards.len() == 5 {
            return GameState::PlayerWon;
        };
        if player_score == 21 {
            return GameState::PlayerWon;
        };
        if dealer_score == 21 {
            return GameState::PlayerLost;
        }
        if !(self.player_stay_status || self.dealer_stay_status) {
            return GameState::InProgress;
        };
        if player_score == dealer_score {
            return GameState::PlayerLost;
        };
        if player_score > 21 {
            return GameState::PlayerLost;
        };
        if dealer_score > 21 {
            return GameState::PlayerWon;
        };
        if player_score > dealer_score {
            return GameState::PlayerWon;
        }
        if player_score < dealer_score {
            return GameState::PlayerLost;
        }
        GameState::InProgress
    }

    // Computes dealer play
    pub fn dealer_play(&mut self) -> Result<(), &'static str> {
        if !self.player_stay_status {
            return Err("Player is not done yet");
        }
        self.first_turn = false;
        while self.status() == GameState::InProgress && self.dealer.score() < 17 {
            self.dealer_hit()?; // No errors should happen here
        }
        self.dealer_stay();
        Ok(())
    }

    pub fn save(&self) {
        let conn = self.db_pool.clone().get().unwrap();
        let (game_status, bet): (Option<bool>, Option<i64>) = match self.status() {
            GameState::InProgress => (None, Some(self.bet as i64)),
            GameState::PlayerWon => (Some(true), None),
            GameState::PlayerLost => (Some(false), None),
        };
        let sess = Session {
            id: self.player_id as i64,
            bet: bet,
            dealer_hand: self.dealer.export().1,
            dealer_stay: self.dealer_stay_status,
            deck: self.deck.export(),
            first_turn: self.first_turn,
            player_hand: self.player.export().1,
            player_stay: self.player_stay_status,
            status: game_status,
        };
        // TODO: make this safe
        let _: Session = sess.save_changes(&*conn).unwrap();
    }

    fn db_remove(&self) {
        use schema::blackjack::dsl::*;
        let conn = self.db_pool.clone().get().unwrap();
        let _num = diesel::delete(blackjack.filter(id.eq(self.player_id as i64)))
            .execute(&*conn)
            .expect("Error deleting Previous BlackJack Test data");
    }

    /// Consumes session and returns Gain
    pub fn claim(mut self) -> Result<Self, Self> {
        match self.status() {
            GameState::InProgress => Err(self),
            GameState::PlayerWon => {
                self.claimed = true;
                self.gain = self.bet as i64;
                Ok(self)
            }
            GameState::PlayerLost => {
                self.claimed = true;
                self.gain = -(self.bet as i64);
                Ok(self)
            }
        }
    }
}


impl Drop for BlackJack {
    fn drop(&mut self) {
        if !self.claimed {
            self.save(); // Save before vanishing
        } else {
            self.db_remove();
        }
    }
}
