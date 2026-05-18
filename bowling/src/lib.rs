#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    rolls: [Option<u8>; 22],
    current_roll: usize,
    pins_left: u8,
    rolls_left: u8,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            rolls: [None; 22],
            current_roll: 0,
            pins_left: 0,
            rolls_left: 20,
        }
    }

    pub fn roll(&mut self, pins_scored: u8) -> Result<(), Error> {
        let is_first_in_frame = self.current_roll.is_multiple_of(2);
        if self.rolls_left == 0 {
            return Err(Error::GameComplete);
        }
        // reset pins when starting new frame or if strike was hit in the 1st bonus roll
        if is_first_in_frame || (self.current_roll == 21 && self.pins_left == 0) {
            self.pins_left = 10;
        }
        if pins_scored > self.pins_left {
            return Err(Error::NotEnoughPinsLeft);
        }

        self.pins_left -= pins_scored;
        self.rolls[self.current_roll] = Some(pins_scored);
        self.rolls_left -= 1;

        if self.pins_left == 0 {
            if is_first_in_frame { // strike
                // close the frame if it's not bonus roll
                if !self.is_bonus_roll() {
                    self.current_roll += 1;
                    self.rolls_left -= 1;
                    if self.rolls_left < 2 {
                        self.rolls_left = 2;
                    }
                }
            } else { // spare
                if self.rolls_left < 1 && !self.is_bonus_roll() {
                    self.rolls_left = 1;
                }
            }
        }

        self.current_roll += 1;
        Ok(())
    }

    fn is_bonus_roll(&self) -> bool { self.current_roll >= 20}

    pub fn score(&self) -> Option<u16> {
        if self.rolls_left > 0 {
            return None;
        }

        let mut total_score: u16 = 0;
        let mut rolls_history = vec![self.rolls[21].unwrap_or_default(),self.rolls[20].unwrap_or_default()];

        for frame in self.rolls[..20].chunks(2).rev() {
            match frame {
                // spare or regular
                [Some(a), Some(b)] => {
                    if a + b == 10 { // spare
                        total_score += rolls_history[rolls_history.len() - 1] as u16;
                    }

                    total_score += (a + b) as u16;
                    rolls_history.push(*b);
                    rolls_history.push(*a);
                }
                // strike
                [Some(a), None] => {
                    total_score += (a + rolls_history.iter().rev().take(2).sum::<u8>()) as u16;
                    rolls_history.push(*a);
                }
                _ => ()
            }
        }

        Some(total_score)
    }
}
