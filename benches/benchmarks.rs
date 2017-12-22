#![feature(test)]

extern crate games_microservice;
extern crate test;

#[cfg(feature = "auto_save")]
mod blackjack {
    use games_microservice::games::blackjack::BlackJack;
    use games_microservice::establish_connection_pool;
    use test::Bencher;

    #[bench]
    fn bench_mark(b: &mut Bencher) {
        use games_microservice::games::blackjack::BlackJackError::*;
        let pool = establish_connection_pool();
        let mut uid = 10_000_000;

        b.iter(move || {
            uid += 1;

            {
                let mut bj = BlackJack::new(uid, 0, pool.clone()).expect("Failed to create BlackJack Session");
                match bj.player_hit() {
                    Ok(_) => bj.player_stay().expect("Player Failed to stay"),
                    Err(DealerAlreadyWon) | Err(PlayerAlreadyWon) | Err(DealerAlreadyLost) | Err(PlayerAlreadyLost) => (),
                    Err(e) => panic!(e)
                }
            }
            BlackJack::restore(&pool, uid).expect("Restore failed").claim().ok();
        })
    }
}

#[cfg(not(feature = "auto_save"))]
mod blackjack {
    use test::Bencher;
    use games_microservice::games::blackjack::BlackJack;

    #[bench]
    fn bench_mark(b: &mut Bencher) {
        b.iter(move || {
            let mut bj = BlackJack::new(100).expect("Failed to create blackjack session");
            match bj.player_hit() {
                Ok(_) => bj.player_stay().expect("Player failed to stay"),
                Err(_) => ()
            }
            bj.claim().expect("Failed to make claim");
        })
    }
}

mod coin_toss {
    use test::Bencher;
    use games_microservice::games::coin_toss::guess_side;

    #[bench]
    fn bench_coin(bench: &mut Bencher) {
        bench.iter(|| guess_side(0, "h"))
    }
}

mod rps_game {
    use test::Bencher;
    use games_microservice::games::rps::rps;

    #[bench]
    fn bench_rps(b: &mut Bencher) {
        b.iter(|| rps(100, "r"))
    }
}

mod slot_machine {
    use games_microservice::games::slot_machine::SlotMachine;
    use test::Bencher;

    #[bench]
    fn test_slot_machine(b: &mut Bencher) {
        b.iter(|| {
            let gain = SlotMachine::new(100).gain;

            assert!([-100, 50, 100].iter().any(|i| *i == gain));
        })
    }
}
