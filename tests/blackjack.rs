extern crate games_microservice;
use games_microservice::games::blackjack::*;
//use games_microservice::establish_connection_pool;

#[test]
fn test_blackjack_deck() {
    let mut deck = Deck::new();
    assert_eq!(deck.export().len(), 13 * 4);
    match deck.draw() {
        Ok(card) => {
            card.to_string();
        }
        Err(e) => panic!(e.to_string()),
    }
}

//#[test]
//fn test_blackjack_save_and_claim() {
//	let pool = establish_connection_pool();
//	let bj = BlackJack::new(22, 500, pool.clone());
//	match bj {
//		Err(e) => panic!(e.to_string()),
//		Ok(mut bj) => {
//			match bj.save() {
//				Ok(_) => (),
//				Err(e) => panic!(e.to_string())
//			}
//			bj.player_stay().ok();
//			match bj.claim() {
//				Ok(_) => (),
//				Err(e) => panic!(e.to_string())
//			}
//
//		}
//	}
//}
