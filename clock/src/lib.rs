use std::fmt::{Debug, Display};

const MINUTES_IN_DAY: i32 = 24 * 60;
#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let normalized_minutes = (hours * 60 + minutes) % (MINUTES_IN_DAY);
        let normalized_minutes = if normalized_minutes < 0 {MINUTES_IN_DAY + normalized_minutes} else {normalized_minutes};

        Clock {hours: normalized_minutes / 60 % 24, minutes: normalized_minutes % 60}
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}