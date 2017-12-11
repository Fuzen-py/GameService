use regex::Regex;
use std::char::ParseCharError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;

#[derive(Clone, Debug)]
pub struct Card {
    pub name: &'static str,
    pub value: u8,
    pub symbol: &'static str,
}

const NON_ROYALTY_CARDS: [&str; 10] = [
    "ACE", "TWOS", "THREES", "FOURS", "FIVES", "SIXES", "SEVENS", "EIGHTS", "NINES", "TENS"
];

const VALID_SYMBOLS: [&str; 4] = ["HEARTS", "SPADES", "CLUBS", "DIAMONDS"];

const ROYALTY_CARDS: [&str; 3] = ["JACKS", "KINGS", "QUEENS"];

#[derive(Clone, Debug)]
pub enum CardParseError {
    InvalidCard,
    NoCaptureGroup,
    NoSymbol,
    ParseChar(ParseCharError),
}

impl Display for CardParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for CardParseError {
    fn description(&self) -> &str {
        use self::CardParseError::*;

        match *self {
            InvalidCard => "Invalid card given",
            NoCaptureGroup => "No regex capture group matched",
            NoSymbol => "No matching symbol found",
            ParseChar(ref inner) => inner.description(),
        }
    }
}

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static!{
            static ref RE: Regex = Regex::new(r"(.*):(.*)").unwrap();
        }

        let data = RE.captures_iter(s)
            .next()
            .ok_or_else(|| CardParseError::NoCaptureGroup)?;

        let symbol_pos = VALID_SYMBOLS
            .iter()
            .position(|&r| r == &data[1])
            .ok_or_else(|| CardParseError::NoSymbol)?;

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
                    None => return Err(CardParseError::InvalidCard),
                },
            },
        )
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}:{}", self.symbol, self.name)
    }
}
