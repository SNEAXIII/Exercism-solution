// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration{
    seconds: u64
}

const DAY_YEAR: f64= 365.25;
const HOUR_DAY:f64= 24.0;
const MINUTE_HOUR:f64= 60.0;
const SECOND_MINUTE:f64= 60.0;
const SECOND_YEAR:f64= SECOND_MINUTE * MINUTE_HOUR * HOUR_DAY * DAY_YEAR;

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Duration{seconds}
    }
}

pub trait Planet {
    const RATIO: f64;
}

pub trait PlanetExt {
    fn years_during(d: &Duration) -> f64;
}

impl<T> PlanetExt for T where T: Planet{
    fn years_during(duration: &Duration) -> f64 {
        duration.seconds as f64 / SECOND_YEAR / T::RATIO
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
    const RATIO: f64 = 0.2408467;
}
impl Planet for Venus {
    const RATIO: f64 = 0.61519726;
}
impl Planet for Earth {
    const RATIO: f64 = 1.0;
}
impl Planet for Mars {
    const RATIO: f64 = 1.8808158;
}
impl Planet for Jupiter {
    const RATIO: f64 = 11.862615;
}
impl Planet for Saturn {
    const RATIO: f64 = 29.447498;
}
impl Planet for Uranus {
    const RATIO: f64 = 84.016846;
}
impl Planet for Neptune {
    const RATIO: f64 = 164.79132;
}
