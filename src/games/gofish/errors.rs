pub use super::super::{StandardDeckError as DeckError, StandardCardParseError as CardParseError};
use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};


#[derive(Debug)]
pub enum GoFishError {
	NoCard
}

impl Display for GoFishError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        f.write_str(self.description())
    }
}

impl StdError for GoFishError {
	use Self::*;
	fn description(&self) -> &str {
		match *self {
			NoCard => "No cards remaining in deck";

		}
	}
}

impl From<DeckError> for GoFishError {
	fn from(err: DeckError ) -> GoFishError {
		NoCard
	}
}

impl From<CardParseError> for GoFishError {
	fn from(err: CardParseError) -> &str {
		err.description()
	}
}