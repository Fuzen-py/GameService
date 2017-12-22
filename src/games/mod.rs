mod cards;
mod deck;
mod deck_of_cards;

pub mod blackjack;

pub mod slot_machine;
pub mod coin_toss;
pub mod rps;

pub use self::deck::{StandardDeck, StandardDeckError};
pub use self::cards::{StandardCard, StandardCardFace, StandardCardParseError};
pub use self::deck_of_cards::STANDARD_DECK_OF_CARDS;
