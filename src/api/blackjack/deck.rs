use api::blackjack::{BlackJackError, Card, DECK_OF_CARDS};

#[derive(Clone, Debug, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    #[allow(unused_mut)]
    pub fn new() -> Self {
        let mut cards = DECK_OF_CARDS.to_vec();

        #[cfg(not(any(test, bench)))]
        {
            use rand::{thread_rng, Rng};
            thread_rng().shuffle(&mut cards);
        }

        Self { cards }
    }

    pub fn draw(&mut self) -> Result<Card, BlackJackError> {
        // Game should never get to the point where the deck is empty
        match self.cards.pop() {
            Some(card) => Ok(card),
            None => Err(BlackJackError::NoCard),
        }
    }

    pub fn export(&self) -> Vec<String> {
        c![card.to_string(), for card in &self.cards]
    }
}
