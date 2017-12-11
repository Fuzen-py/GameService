extern crate serde_json;
extern crate test;

use api::blackjack::Response;
use endpoints::router;
use rocket;
use rocket::local::Client;
use serde_json::Value;
use self::test::Bencher;
use establish_connection_pool;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActiveSessionsCount {
    active_sessions: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ActiveSessions {
    pub status_code: u16,
    pub status: Result<ActiveSessionsCount, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Claim {
    pub status_code: u16,
    pub status: Result<u64, String>,
}

fn create_client(use_db: bool) -> Client {
    if use_db {
        Client::new(router(
            rocket::ignite().manage(establish_connection_pool().clone()),
        )).unwrap()
    } else {
        Client::new(router(rocket::ignite())).unwrap()
    }
}

#[test]
fn test_blackjack_routes() {
    let client = create_client(true);

    // Test session counter (should be 0)
    {
        let mut resp = client.get("/blackjack/").dispatch();
        let resp: ActiveSessions = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        assert_eq!(
            resp.status
                .expect("/blackjack/: Session count returned an error")
                .active_sessions,
            0
        );
    }

    // Test Creation and info route
    {
        let mut resp = client.post("/blackjack/0/create/1").dispatch();
        let resp: Response = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        let resp = resp.status
            .expect("An Error has occurred on session creation");
        assert_eq!(resp.dealer_hand.len(), 1);
        assert!(resp.game_state.is_none());
        let mut resp = client.get("/blackjack/0").dispatch();
        let resp: Response = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(
            resp.status
                .expect("An Error has occurred on session creation")
                .game_state
                .is_none()
        );
    }
    // Test Creation route fails
    {
        let mut resp = client.post("/blackjack/0/create/1").dispatch();
        let resp: Response = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 501);
    }
    // Test Hit Route
    {
        let mut resp = client.post("/blackjack/0/hit").dispatch();
        let resp: Response = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(match resp.status {
            Ok(stat) => stat.dealer_hand.len() >= 2,
            Err(_) => true,
        });
    }
    // Test Stay Route
    {
        let mut resp = client.post("/blackjack/0/stay").dispatch();
        let resp: Response = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        assert!(
            resp.status
                .expect("/blackjack/<user>/stay/: Failed on first stay.")
                .game_state
                .is_some()
        );
    }
    // Test Hit doesn't work
    {
        let mut resp = client.post("/blackjack/0/hit").dispatch();
        let resp: Response = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 501);
    }
    // Test claim route works
    {
        let mut resp = client.get("/blackjack/0").dispatch();
        let resp: Response = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        assert_eq!(resp.status_code, 200);
        let status = resp.status.unwrap();
        let mut expected_gain = -1;
        if status.game_state.unwrap() {
            expected_gain = 1;
        }
        let mut resp = client.post("/blackjack/0/claim").dispatch();
        let resp: Response = serde_json::from_str(&resp.body_string().unwrap()).unwrap();
        let status_code: u16 = resp.status_code as u16;
        let returned_gain: i64 = resp.status.unwrap().gain;
        assert_eq!(status_code, 200);
        assert_eq!(returned_gain, expected_gain);
    }
}

#[bench]
fn bench_blackjack_routes(b: &mut Bencher) {
    let client = create_client(true);

    b.iter(|| {
        client.post("/blackjack/16/create/1").dispatch();
        client.post("/blackjack/16/stay").dispatch();
        client.post("/blackjack/16/claim").dispatch();
    })
}

#[bench]
fn test_slot_route(b: &mut Bencher) {
    let client = create_client(false);

    b.iter(|| {
        let mut resp = client.get("/slot_machine/23").dispatch();
        let resp = serde_json::from_str::<Value>(&resp.body_string().unwrap()).unwrap();
        let ret: &i64 = &resp["status"]["Ok"]["gain"].as_i64().unwrap();

        if !vec![-23, 11, 23].iter().any(|i| i == ret) {
            panic!("SLOTS DID BAD MATH");
        }
    })
}

#[bench]
fn bench_coin_toss_route(b: &mut Bencher) {
    let client = create_client(false);
    b.iter(|| client.get("/coin_toss/h/100").dispatch())
}
