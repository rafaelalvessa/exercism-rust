/// Number of minutes in an hour.
const MINUTES_IN_HOUR: u32 = 60;

/// The number of cars produced each hour at the lowest speed, 1.
const TOTAL_CARS_PER_HOUR: u32 = 221;

/// Returns the number of working cars for the given speed.
pub fn production_rate_per_hour(speed: u8) -> f64 {
    TOTAL_CARS_PER_HOUR as f64 * speed as f64 * success_rate(speed)
}

/// Returns the success rate for the given speed.
///
/// The success rate is 0% for any speed outside the range between 1 and 10, inclusive.
fn success_rate(speed: u8) -> f64 {
    match speed {
        1..=4 => 1.0,
        5..=8 => 0.9,
        9..=10 => 0.77,
        _ => 0.0,
    }
}

/// Returns the number of working cars produced in a minute for the given speed.
pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / MINUTES_IN_HOUR as f64).floor() as u32
}
