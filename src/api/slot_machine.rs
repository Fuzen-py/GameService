use rand::{thread_rng, Rng};

// Returns multiplier
const ROW: [&str; 7] = ["🍒", "🍊", "🍓", "🍍", "🍇", "🍉", "⭐"];
const ROW_LEN: usize = 7;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SlotMachine {
    pub picks: Vec<String>,
    pub bet: u64,
    pub gain: i64,
}

impl SlotMachine {
    pub fn new(bet: u64) -> Self {
        let mut choices = Vec::with_capacity(3);
        let mut rng = thread_rng();

        for _ in 0 .. 3 {
            choices.push(ROW[rng.gen_range(0, ROW_LEN)].to_string());
        }

        let picks = choices.clone();

        #[cfg(test)]
        {
            assert_eq!(picks.len(), 3);
        }

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
