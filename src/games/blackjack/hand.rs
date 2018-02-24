use super::Card;
use super::blackjack_game::{card_suit, card_value};


/// Player Card hand
#[derive(Clone, Debug, Default)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Self {
        Self {
            cards: Vec::with_capacity(5),
        }
    }

    /// Add a card to the hand
    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card)
    }

    /// Calculate the score
    pub fn score(&self) -> u64 {
        let mut ace_count = 0u64;
        let mut total = 0u64;

        for card in &self.cards {
            if card_suit(card) == "Ace" {
                ace_count += 1;
            } else {
                total += u64::from(card_value(card));
            }
        }

        if ace_count >= 1 {
            if total <= 10 {
                total += 11
            } else {
                total += 1
            };

            ace_count -= 1;
        };

        total + ace_count
    }

    /// Exports the hand (Score, Vec<Cards as string>)
    /// Note: Doesnt consume self
    pub fn export(&self) -> (u64, Vec<String>) {
        (self.score(), c![card.to_string(), for card in &self.cards])
    }
}
