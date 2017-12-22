use super::StandardCard::*;
use super::StandardCard;
use super::StandardCardFace::*;

pub const STANDARD_DECK_OF_CARDS: [StandardCard; 52] = [
    Hearts(Ace),
    Hearts(Two),
    Hearts(Three),
    Hearts(Four),
    Hearts(Five),
    Hearts(Six),
    Hearts(Seven),
    Hearts(Eight),
    Hearts(Nine),
    Hearts(Ten),
    Hearts(Jack),
    Hearts(Queen),
    Hearts(King),
    Spades(Ace),
    Spades(Two),
    Spades(Three),
    Spades(Four),
    Spades(Five),
    Spades(Six),
    Spades(Seven),
    Spades(Eight),
    Spades(Nine),
    Spades(Ten),
    Spades(Jack),
    Spades(Queen),
    Spades(King),
    Clubs(Ace),
    Clubs(Two),
    Clubs(Three),
    Clubs(Four),
    Clubs(Five),
    Clubs(Six),
    Clubs(Seven),
    Clubs(Eight),
    Clubs(Nine),
    Clubs(Ten),
    Clubs(Jack),
    Clubs(Queen),
    Clubs(King),
    Diamonds(Ace),
    Diamonds(Two),
    Diamonds(Three),
    Diamonds(Four),
    Diamonds(Five),
    Diamonds(Six),
    Diamonds(Seven),
    Diamonds(Eight),
    Diamonds(Nine),
    Diamonds(Ten),
    Diamonds(Jack),
    Diamonds(Queen),
    Diamonds(King),
];
