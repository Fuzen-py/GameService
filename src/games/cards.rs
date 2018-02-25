use regex::Regex;
use std::char::ParseCharError;
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::str::FromStr;
use std::convert::From;
#[derive(Clone, Debug, Copy)]
pub enum StandardCardFace {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    King,
    Queen,
    Joker,
}

impl Display for StandardCardFace {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::StandardCardFace::*;
        let text_value = match *self {
            Ace => "ACE",
            Two => "TWO",
            Three => "THREE",
            Four => "FOUR",
            Five => "FIVE",
            Six => "SIX",
            Seven => "SEVEN",
            Eight => "Eight",
            Nine => "NINE",
            Ten => "TEN",
            Jack => "JACK",
            King => "KING",
            Queen => "QUEEN",
            Joker => "JOKER",
        };
        write!(f, "{}", text_value)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum StandardCard {
    Hearts(StandardCardFace),
    Spades(StandardCardFace),
    Clubs(StandardCardFace),
    Diamonds(StandardCardFace),
}

impl Display for StandardCard {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::StandardCard::*;
        write!(
            f,
            "{}",
            match *self {
                Hearts(ref inner) => format!("HEARTS:{}", inner.to_string()),
                Spades(ref inner) => format!("SPADES:{}", inner.to_string()),
                Clubs(ref inner) => format!("CLUBS:{}", inner.to_string()),
                Diamonds(ref inner) => format!("DIAMONDS:{}", inner.to_string()),
            }
        )
    }
}

impl FromStr for StandardCard {
    type Err = StandardCardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let txt = s.to_uppercase();
        use self::StandardCard::*;
        lazy_static!{
            static ref RE: Regex = Regex::new(r"(.*):(.*)").unwrap();
        }

        let data = RE.captures_iter(&txt)
            .next()
            .ok_or_else(|| StandardCardParseError::NoCaptureGroup)?;

        let face = match &data[2] {
            "ACE" => StandardCardFace::Ace,
            "TWO" => StandardCardFace::Two,
            "THREE" => StandardCardFace::Three,
            "FOUR" => StandardCardFace::Four,
            "FIVE" => StandardCardFace::Five,
            "SIX" => StandardCardFace::Six,
            "SEVEN" => StandardCardFace::Seven,
            "EIGHT" => StandardCardFace::Eight,
            "NINE" => StandardCardFace::Nine,
            "TEN" => StandardCardFace::Ten,
            "JACK" => StandardCardFace::Jack,
            "KING" => StandardCardFace::King,
            "QUEEN" => StandardCardFace::Queen,
            "JOKER" => StandardCardFace::Joker,
            _ => return Err(StandardCardParseError::InvalidCard),
        };
        Ok(match &data[1] {
            "HEARTS" => Hearts(face),
            "SPADES" => Spades(face),
            "CLUBS" => Clubs(face),
            "DIAMONDS" => Diamonds(face),
            _ => return Err(StandardCardParseError::NoSymbol),
        })
    }
}

#[derive(Clone, Debug)]
pub enum StandardCardParseError {
    InvalidCard,
    NoCaptureGroup,
    NoSymbol,
    ParseChar(ParseCharError),
}

impl Display for StandardCardParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for StandardCardParseError {
    fn description(&self) -> &str {
        use self::StandardCardParseError::*;

        match *self {
            InvalidCard => "Invalid card given",
            NoCaptureGroup => "No regex capture group matched",
            NoSymbol => "No matching symbol found",
            ParseChar(ref inner) => inner.description(),
        }
    }
}
impl StandardCard {
    pub fn face(&self) -> StandardCardFace {
        self.clone().into()
    }
    pub fn suite_string(&self) -> String {
        use self::StandardCard::*;
        String::from(match *self {
            Hearts(_) => "Hearts",
            Clubs(_) => "Clubs",
            Spades(_) => "Spades",
            Diamonds(_) => "Diamonds",
        })
    }
    pub fn face_as_string(&self) -> String {
        self.face().to_string()
    }
}

impl StandardCardFace {
    /// Returns the numerical value of a Face by blackjack standards
    /// Please note, Ace can be both 11 and 1 but is 11 in this instance
    pub fn value(&self) -> u8 {
        self.clone().into()
    }
}

impl From<StandardCard> for StandardCardFace {
    fn from(card: StandardCard) -> StandardCardFace {
        use self::StandardCard::*;
        match card {
            Hearts(inner) | Spades(inner) | Clubs(inner) | Diamonds(inner) => inner,
        }
    }
}

impl From<StandardCardFace> for u8 {
    fn from(face: StandardCardFace) -> u8 {
        use self::StandardCardFace::*;
        match face {
            Ace => 11,
            Two => 2,
            Three => 3,
            Four => 4,
            Five => 5,
            Six => 6,
            Seven => 7,
            Eight => 8,
            Nine => 9,
            _ => 10,
        }
    }
}

impl From<StandardCard> for u8 {
    fn from(card: StandardCard) -> u8 {
        u8::from(StandardCardFace::from(card))
    }
}

impl From<StandardCard> for u64 {
    fn from(card: StandardCard) -> u64 {
        u64::from(u8::from(card))
    }
}
