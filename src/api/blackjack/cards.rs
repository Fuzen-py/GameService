extern crate regex;
use self::regex::Regex;
use std::fmt;
use std::str::FromStr;
use std::char::ParseCharError;

#[derive(Clone, Debug)]
pub struct Card {
    pub name: &'static str,
    pub value: u8,
    pub symbol: &'static str,
}

const NON_ROYALTY_CARDS: [&str; 10] = [
    "ACE",
    "TWOS",
    "THREES",
    "FOURS",
    "FIVES",
    "SIXES",
    "SEVENS",
    "EIGHTS",
    "NINES",
    "TENS",
];

const VALID_SYMBOLS: [&str; 4] = ["HEARTS", "SPADES", "CLUBS", "DIAMONDS"];

const ROYALTY_CARDS: [&str; 3] = ["JACKS", "KINGS", "QUEENS"];

impl FromStr for Card {
    type Err = ParseCharError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"(.*):(.*)").unwrap();
        }

        let data = RE.captures_iter(s).next().unwrap();
        let symbol_pos = VALID_SYMBOLS.iter().position(|&r| r == &data[1]).unwrap();

        Ok(
            match NON_ROYALTY_CARDS.iter().position(|&r| r == &data[2]) {
                Some(position) => Card {
                    name: NON_ROYALTY_CARDS[position],
                    value: position as u8 + 1u8,
                    symbol: VALID_SYMBOLS[symbol_pos],
                },
                None => match ROYALTY_CARDS.iter().position(|&r| r == &data[2]) {
                    Some(position) => Card {
                        name: ROYALTY_CARDS[position],
                        value: 10u8,
                        symbol: VALID_SYMBOLS[symbol_pos],
                    },
                    None => unreachable!(), // If this reached, fix your data
                },
            },
        )
    }
}


impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.symbol, self.name)
    }
}
