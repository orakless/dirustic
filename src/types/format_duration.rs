use std::fmt::{Display, Formatter};
use std::time::Duration;

const SECONDS_IN_HOUR: u64 = 3600;
const SECONDS_IN_MINUTE: u64 = 60;

pub struct FormatDuration {
    hours: u64,
    minutes: u8,
    seconds: u8
}

impl FormatDuration {
    pub fn new(seconds: u64) -> Self {
        if seconds == 0 {
            return Self { hours: 0, minutes: 0, seconds: 0 };
        }

        let hours = seconds / SECONDS_IN_HOUR;

        // it will be < to 60 because of last operation
        let minutes = ((seconds - (hours * SECONDS_IN_HOUR)) / SECONDS_IN_MINUTE);

        // it will be < to 60 because of last operation
        let seconds = (seconds - (hours * SECONDS_IN_HOUR) - (minutes * SECONDS_IN_MINUTE));

        Self {
            hours,
            minutes: minutes as u8,
            seconds: seconds as u8
        }
    }
}

impl From<Duration> for FormatDuration {
    fn from(duration: Duration) -> Self {
        FormatDuration::new(duration.as_secs())
    }
}

impl Display for FormatDuration {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{:02}:{:02}", self.hours, self.minutes, self.seconds)
    }
}
