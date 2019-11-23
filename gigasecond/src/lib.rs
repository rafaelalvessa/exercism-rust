use chrono::{DateTime, Utc, Duration};

const GIGASEC_IN_SEC: i64 = 1_000_000_000;

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    start + Duration::seconds(GIGASEC_IN_SEC)
}
