use std::fmt;

const DAY: i32 = 60 * 24;
const HOUR: i32 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (DAY + (hours * HOUR + minutes) % DAY) % DAY;
        let hour = total_minutes / 60;
        let minute = total_minutes % 60;

        Self { hour, minute }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hour, self.minute + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}


