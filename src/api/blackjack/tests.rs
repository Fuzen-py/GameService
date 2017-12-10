extern crate test;
use establish_connection_pool;
use api::blackjack::BlackJack;
use self::test::Bencher;

#[bench]
fn bench_mark(b: &mut Bencher) {
    let pool = establish_connection_pool();
    let mut uid = 10000;

    b.iter(move || {
        uid += 1;

        {
            let mut bj = BlackJack::new(uid, 0, pool.clone()).unwrap();
            bj.player_hit().unwrap();
            bj.player_stay();
        }

        BlackJack::restore(pool.clone(), uid).unwrap().claim().ok();
    })
}
