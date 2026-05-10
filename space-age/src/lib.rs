// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.
const DAYS_IN_EARTH_YEAR: f64 = 365.25;
const SECONDS_IN_EARTH_YEAR: f64 = DAYS_IN_EARTH_YEAR * 24.0 * 60.0 * 60.0;

#[derive(Debug)]
pub struct Duration {
    earth_years: f64
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            earth_years: s as f64 / SECONDS_IN_EARTH_YEAR,
        }
    }
}

pub trait Planet {
    fn earth_year_ratio() -> f64;

    fn years_during(d: &Duration) -> f64 {
        d.earth_years / Self::earth_year_ratio()
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

impl Planet for Mercury {
    fn earth_year_ratio() -> f64 { 0.2408467 }
}

impl Planet for Venus {
    fn earth_year_ratio() -> f64 { 0.61519726 }
}

impl Planet for Earth {
    fn earth_year_ratio() -> f64 { 1.0 }
}

impl Planet for Mars {
    fn earth_year_ratio() -> f64 { 1.8808158 }
}

impl Planet for Jupiter {
    fn earth_year_ratio() -> f64 { 11.862615 }
}

impl Planet for Saturn {
    fn earth_year_ratio() -> f64 { 29.447498 }
}

impl Planet for Uranus {
    fn earth_year_ratio() -> f64 { 84.016846 }
}

impl Planet for Neptune {
    fn earth_year_ratio() -> f64 { 164.79132 }
}
