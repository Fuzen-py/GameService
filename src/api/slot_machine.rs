use rand::{thread_rng, Rng};

// Returns multiplier
const ROW: [&str; 7] = ["🍒", "🍊", "🍓", "🍍", "🍇", "🍉", "⭐"];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SlotMachine {
    pub picks: Vec<String>,
    pub bet: u64,
    pub gain: i64,
}

impl SlotMachine {
    pub fn new(bet: u64) -> Self {
        let mut choices: Vec<String> = Vec::with_capacity(3);
        let mut rng = thread_rng();
        let row_len = ROW.len();

        choices.push(ROW[rng.gen_range(0, row_len)].to_string());
        choices.push(ROW[rng.gen_range(0, row_len)].to_string());
        choices.push(ROW[rng.gen_range(0, row_len)].to_string());

        let picks = choices.clone();

        #[cfg(test)]
        assert!(picks.len() == 3);

        choices.sort();
        choices.dedup();

        let gain = match choices.len() {
            3 => -(bet as i64),
            2 => (bet / 2) as i64,
            1 => bet as i64,
            _ => unreachable!(),
        };

        Self {
            bet,
            gain,
            picks,
        }
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Response {
    pub status_code: u16,
    pub status: Result<SlotMachine, ()>,
}

#[cfg(any(test, bench))]
mod test {
    extern crate test;

    use api::slot_machine::SlotMachine;
    use self::test::Bencher;

    #[bench]
    fn test_slot_machine(b: &mut Bencher) {
        b.iter(|| {
            let gain = SlotMachine::new(100).gain;

            assert!([-100, 50, 100].iter().any(|i| *i == gain));
        })
    }
}
