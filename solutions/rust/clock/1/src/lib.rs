use std::fmt::{Debug, Display};

pub struct Clock {
    hours: i32,
    minutes: i32
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let normalized_minutes = (hours * 60 + minutes) % (24 * 60);
        let normalized_minutes = if normalized_minutes < 0 {24 * 60 + normalized_minutes} else {normalized_minutes};

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

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        self.hours == other.hours && self.minutes == other.minutes
    }
}

impl Debug for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Clock >> {:02}:{:02}", self.hours, self.minutes)
    }
}
