use api::blackjack::Card;

macro_rules! cards {
    ($($name:expr, $value:expr, $symbol:expr;)*) => ([
        $(
            Card {
                name: $name,
                symbol: $symbol,
                value: $value,
            },
        )*
    ]);
}

pub const DECK_OF_CARDS: [Card; 52] = cards! {
    "ACE", 11, "HEARTS";
    "TWOS", 2, "HEARTS";
    "THREES", 3, "HEARTS";
    "FOURS", 4, "HEARTS";
    "FIVES", 5, "HEARTS";
    "SIXES", 6, "HEARTS";
    "SEVENS", 7, "HEARTS";
    "EIGHTS", 8, "HEARTS";
    "NINES", 9, "HEARTS";
    "TENS", 10, "HEARTS";
    "JACKS", 10, "HEARTS";
    "KINGS", 10, "HEARTS";
    "QUEENS", 10, "HEARTS";
    "ACE", 11, "SPADES";
    "TWOS", 2, "SPADES";
    "THREES", 3, "SPADES";
    "FOURS", 4, "SPADES";
    "FIVES", 5, "SPADES";
    "SIXES", 6, "SPADES";
    "SEVENS", 7, "SPADES";
    "EIGHTS", 8, "SPADES";
    "NINES", 9, "SPADES";
    "TENS", 10, "SPADES";
    "JACKS", 10, "SPADES";
    "KINGS", 10, "SPADES";
    "QUEENS", 10, "SPADES";
    "ACE", 11, "CLUBS";
    "TWOS", 2, "CLUBS";
    "THREES", 3, "CLUBS";
    "FOURS", 4, "CLUBS";
    "FIVES", 5, "CLUBS";
    "SIXES", 6, "CLUBS";
    "SEVENS", 7, "CLUBS";
    "EIGHTS", 8, "CLUBS";
    "NINES", 9, "CLUBS";
    "TENS", 10, "CLUBS";
    "JACKS", 10, "CLUBS";
    "KINGS", 10, "CLUBS";
    "QUEENS", 10, "CLUBS";
    "ACE", 11, "DIAMONDS";
    "TWOS", 2, "DIAMONDS";
    "THREES", 3, "DIAMONDS";
    "FOURS", 4, "DIAMONDS";
    "FIVES", 5, "DIAMONDS";
    "SIXES", 6, "DIAMONDS";
    "SEVENS", 7, "DIAMONDS";
    "EIGHTS", 8, "DIAMONDS";
    "NINES", 9, "DIAMONDS";
    "TENS", 10, "DIAMONDS";
    "JACKS", 10, "DIAMONDS";
    "KINGS", 10, "DIAMONDS";
    "QUEENS", 10, "DIAMONDS";
};
