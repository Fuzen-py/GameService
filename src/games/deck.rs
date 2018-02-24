use super::{StandardCard, STANDARD_DECK_OF_CARDS};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Clone, Debug, Default)]
pub struct StandardDeck {
    pub cards: Vec<StandardCard>,
}

#[derive(Debug)]
pub enum StandardDeckError {
    NoCard,
}

impl Display for StandardDeckError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for StandardDeckError {
    fn description(&self) -> &str {
        match *self {
            StandardDeckError::NoCard => "No Cards are left in the deck",
        }
    }
}

impl StandardDeck {
    #[allow(unused_mut)]
    pub fn new() -> Self {
        let mut cards: Vec<StandardCard> = STANDARD_DECK_OF_CARDS.to_vec();
        use rand::{thread_rng, Rng};
        thread_rng().shuffle(&mut cards);
        Self { cards }
    }

    pub fn draw(&mut self) -> Result<StandardCard, StandardDeckError> {
        // Game should never get to the point where the deck is empty
        match self.cards.pop() {
            Some(card) => Ok(card),
            None => Err(StandardDeckError::NoCard),
        }
    }

    /// Exports a Vector if stringified Cards (Doesnt consume self)
    pub fn export(&self) -> Vec<String> {
        c![card.to_string(), for card in &self.cards]
    }
}
