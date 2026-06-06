use std::collections::VecDeque;
use std::mem;

pub struct RailFence {
    pub rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        Self {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut output = String::new();
        output.reserve(text.len());
        // period of triangle-shaped figure
        let period = 2 * self.rails - 2;

        // iterate through the input rails*text.len() times and pull char by char
        for cur_rail_num in 0..self.rails {
            for (i, c) in text.chars().enumerate() {
                let rem = match i % (period) {
                    x if x < self.rails => x,
                    x => period - x
                };
                if rem == cur_rail_num {
                    output.push(c)
                }
            }
        }

        output
    }

    pub fn decode(&self, cipher: &str) -> String {
        let mut rail_counts = vec![0usize; self.rails];
        let period = 2 * self.rails - 2;

        // count how many items are there in each rail
        for i in 0..cipher.len() {
            let rem = match i % (period) {
                x if x < self.rails => x,
                x => period - x
            };
            rail_counts[rem] += 1;
        }

        // fill rails from the cipher text
        let mut rails = rail_counts.iter()
            .scan(0usize, | st, &x| {
                let skip = mem::replace(st, *st + x);
                Some(cipher.chars().skip(skip).take(x).collect::<VecDeque<_>>())
            })
            .collect::<Vec<_>>();

        // make zigzags through rails and collect chars
        let mut output = String::new();
        for i in 0..cipher.len() {
            let rem = match i % (period) {
                x if x < self.rails => x,
                x => period - x
            };
            output.push(rails[rem].pop_front().unwrap())
        }

        output
    }
}
