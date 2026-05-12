use std::fmt::{Debug, Display};

const MINUTES_IN_DAY: i32 = HOURS_IN_DAY * 60;
const HOURS_IN_DAY: i32 = 24;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let normalized_minutes =
            ((hours % HOURS_IN_DAY) * 60 + (minutes % MINUTES_IN_DAY)).rem_euclid(MINUTES_IN_DAY);
        Clock {
            hours: normalized_minutes / 60 % 24,
            minutes: normalized_minutes % 60,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes % MINUTES_IN_DAY)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
