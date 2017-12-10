#![feature(test)]

extern crate games_microservice;
extern crate test;

mod blackjack {
    use games_microservice::api::blackjack::BlackJack;
    use games_microservice::establish_connection_pool;
    use test::Bencher;

    #[bench]
    fn bench_mark(b: &mut Bencher) {
        let pool = establish_connection_pool();
        let mut uid = 10_000;

        b.iter(move || {
            uid += 1;

            {
                let mut bj = BlackJack::new(uid, 0, pool.clone()).unwrap();
                bj.player_hit().unwrap();
                bj.player_stay().unwrap();
            }

            BlackJack::restore(&pool, uid).unwrap().claim().ok();
        })
    }
}

mod coin_toss {
    use test::Bencher;
    use games_microservice::api::coin_toss::guess_side;

    #[bench]
    fn bench_coin(bench: &mut Bencher) {
        bench.iter(|| guess_side(0, "h"))
    }
}

mod rps_game {
    use test::Bencher;
    use games_microservice::api::rps::rps;

    #[bench]
    fn bench_rps(b: &mut Bencher) {
        b.iter(|| rps(100, "r"))
    }
}

mod slot_machine {
    use games_microservice::api::slot_machine::SlotMachine;
    use test::Bencher;

    #[bench]
    fn test_slot_machine(b: &mut Bencher) {
        b.iter(|| {
            let gain = SlotMachine::new(100).gain;

            assert!([-100, 50, 100].iter().any(|i| *i == gain));
        })
    }
}
