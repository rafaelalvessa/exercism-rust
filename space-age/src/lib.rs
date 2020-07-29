const EARTH_YEAR_IN_SECONDS: f64 = 31557600.0;

/// A duration in seconds.
#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    /// Returns a duration from a given age in seconds.
    fn from(s: u64) -> Self {
        Duration(s as f64)
    }
}

/// Calculates a duration in years on a planet.
pub trait Planet {
    /// The orbital period in seconds.
    const ORBITAL_PERIOD: f64;

    /// Returns the duration in years on this planet.
    fn years_during(d: &Duration) -> f64 {
        d.0 / Self::ORBITAL_PERIOD
    }
}

macro_rules! define_planet {
    // Takes a name and orbital period and creates the implementation for this planet.
    ($name:ident, $orbital_period:expr) => {
        pub struct $name;

        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = EARTH_YEAR_IN_SECONDS * $orbital_period;
        }
    };
}

define_planet!(Mercury, 0.2408467);
define_planet!(Venus, 0.61519726);
define_planet!(Earth, 1.0);
define_planet!(Mars, 1.8808158);
define_planet!(Jupiter, 11.862615);
define_planet!(Saturn, 29.447498);
define_planet!(Uranus, 84.016846);
define_planet!(Neptune, 164.79132);
