use api::blackjack::{DECK_OF_CARDS, Card};

#[cfg(not(any(test, bench)))]
use rand::{thread_rng, Rng};

#[derive(Clone, Debug, Default)]
pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    #[allow(unused_mut)]
    pub fn new() -> Self {
        let mut cards = DECK_OF_CARDS.clone();

        #[cfg(not(test))]
        {
            thread_rng().shuffle(&mut cards);
        }

        Self {
            cards: cards.to_vec(),
        }
    }

    pub fn draw(&mut self) -> Card {
        // Game should never get to the point where the deck is empty
        #[cfg(not(test))]
        {
            let i = thread_rng().gen_range(0, self.cards.len());

            self.cards.remove(i)
        }

        #[cfg(test)]
        {
            self.cards.pop().unwrap()
        }
    }

    pub fn export(&self) -> Vec<String> {
        c![card.to_string(), for card in &self.cards]
    }
}
