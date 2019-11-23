use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock(i32);

const MINS_IN_HOUR: i32 = 60;
const HOURS_IN_DAY: i32 = 24;
const MINS_IN_DAY: i32 = MINS_IN_HOUR * HOURS_IN_DAY;

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        Clock(Self::time_mod(hours * MINS_IN_HOUR + minutes))
    }

    pub fn add_minutes(&self, minutes: i32) -> Self {
        Self(Self::time_mod(self.0 + minutes))
    }

    fn time_mod(minutes: i32) -> i32 {
        (minutes % MINS_IN_DAY + MINS_IN_DAY) % MINS_IN_DAY
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:02}:{:02}",
            self.0 / MINS_IN_HOUR,
            self.0 % MINS_IN_HOUR
        )
    }
}

impl From<String> for Clock {
    fn from(time: String) -> Self {
        let time_components = time.split(':');

        if time_components.clone().count() != 2 {
            panic!("The time must use the HH:MM format.");
        }

        let mut time_components = time_components.map(|component| component.parse().unwrap());

        let (hours, minutes) = (
            time_components.next().unwrap(),
            time_components.next().unwrap(),
        );

        if hours < 0 || hours > 24 {
            panic!("The hours must be a value in the range 00-24.");
        }

        if minutes < 0 || minutes > 59 {
            panic!("The minutes must be a value in the range 00-59.");
        }

        Self::new(hours, minutes)
    }
}
