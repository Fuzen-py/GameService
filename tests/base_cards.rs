extern crate games_microservice;
use games_microservice::games::{StandardCard, StandardDeck};

const CARDS: [&'static str; 56] = [
	"HEARTS:ACE",
	"HEARTS:TWO",
	"HEARTS:THREE",
	"HEARTS:FOUR",
	"HEARTS:FIVE",
	"HEARTS:SIX",
	"HEARTS:SEVEN",
	"HEARTS:EIGHT",
	"HEARTS:NINE",
	"HEARTS:TEN",
	"HEARTS:JACK",
	"HEARTS:KING",
	"HEARTS:QUEEN",
	"HEARTS:JOKER",
	"CLUBS:ACE",
	"CLUBS:TWO",
	"CLUBS:THREE",
	"CLUBS:FOUR",
	"CLUBS:FIVE",
	"CLUBS:SIX",
	"CLUBS:SEVEN",
	"CLUBS:EIGHT",
	"CLUBS:NINE",
	"CLUBS:TEN",
	"CLUBS:JACK",
	"CLUBS:KING",
	"CLUBS:QUEEN",
	"CLUBS:JOKER",
	"SPADES:ACE",
	"SPADES:TWO",
	"SPADES:THREE",
	"SPADES:FOUR",
	"SPADES:FIVE",
	"SPADES:SIX",
	"SPADES:SEVEN",
	"SPADES:EIGHT",
	"SPADES:NINE",
	"SPADES:TEN",
	"SPADES:JACK",
	"SPADES:KING",
	"SPADES:QUEEN",
	"SPADES:JOKER",
	"DIAMONDS:ACE",
	"DIAMONDS:TWO",
	"DIAMONDS:THREE",
	"DIAMONDS:FOUR",
	"DIAMONDS:FIVE",
	"DIAMONDS:SIX",
	"DIAMONDS:SEVEN",
	"DIAMONDS:EIGHT",
	"DIAMONDS:NINE",
	"DIAMONDS:TEN",
	"DIAMONDS:JACK",
	"DIAMONDS:KING",
	"DIAMONDS:QUEEN",
	"DIAMONDS:JOKER"
];

#[test]
fn test_card_from_str() {
	for card in CARDS.iter() {
		card.parse::<StandardCard>().unwrap();
	}
	let cards = StandardDeck::new().export();
	for card in &cards {
		match card.parse::<StandardCard>() {
			Ok(_) => (),
			Err(e) => panic!("{} Raised {}, FirstCard: {}", card, e, &cards[0] == card)
		};
	}

}