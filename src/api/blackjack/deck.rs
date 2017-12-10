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
        let mut cards = DECK_OF_CARDS.to_vec();

        #[cfg(not(test))]
        {
            thread_rng().shuffle(&mut cards);
        }

        Self {
            cards,
        }
    }

    pub fn draw(&mut self) -> Option<Card> {
        // Game should never get to the point where the deck is empty
        #[cfg(not(test))]
        {
            let i = thread_rng().gen_range(0, self.cards.len());

            if self.cards.len() < i {
                Some(self.cards.remove(i))
            } else {
                None
            }
        }

        #[cfg(test)]
        {
            Some(self.cards.pop().unwrap())
        }
    }

    pub fn export(&self) -> Vec<String> {
        c![card.to_string(), for card in &self.cards]
    }
}
