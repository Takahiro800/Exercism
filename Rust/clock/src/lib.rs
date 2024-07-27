use std::fmt;

const DAY: i32 = 60 * 24;
const HOUR: i32 = 60;

#[derive(Debug, PartialEq, Eq)]
pub struct Clock {
    hour: u8,
    minute: u8,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = (hours * HOUR + minutes).rem_euclid(DAY);

        Self {
            hour: (total_minutes / HOUR) as u8,
            minute: (total_minutes % HOUR) as u8,
        }
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self::new(self.hour as i32, self.minute as i32 + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}
