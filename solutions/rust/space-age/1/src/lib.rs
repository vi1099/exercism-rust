// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]

//1. Define the Duration struct (obj), it holds seconds
pub struct Duration {
    seconds: u64
}

//2. Implement the from which just gives back a Duration given the integer bit 64
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration {
            seconds: s
        }
    }
}

//3. Define the trait (aka loosely as an interface) for a plant
// a trait is a shared behavior/functionality
// example, each planet can be converted to years relative to Earth so we can just defaulty define that behavior
pub trait Planet {

    //This is saying, orbital years is a shared attribute and must be provided, usually by the impl itself.
    //So say we are doing 'impl Planet for Mercury' and down below when we say "Self::orbital_years"
    //Self is referring to Mercury which implements the interface Planet.
    //So Self or mercury in this example must supply the constant when we impl
    const orbital_years: f64;
    
    fn years_during(d: &Duration) -> f64 {
        let seconds_as_float = d.seconds as f64;
        let seconds_in_a_year: f64 = 31557600.0;
        let earth_years: f64 = seconds_as_float / seconds_in_a_year;
        let years: f64 = earth_years / Self::orbital_years;
        years
    }
}

//TODO: try implementing macros

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

//4. Passing in the required constant orbital years
impl Planet for Mercury { const orbital_years: f64 = 0.2408467; }
impl Planet for Venus { const orbital_years: f64 = 0.61519726; }
impl Planet for Earth { const orbital_years: f64 = 1.0; }
impl Planet for Mars { const orbital_years: f64 = 1.8808158; }
impl Planet for Jupiter { const orbital_years: f64 = 11.862615; }
impl Planet for Saturn { const orbital_years: f64 = 29.447498; }
impl Planet for Uranus { const orbital_years: f64 = 84.016846; }
impl Planet for Neptune { const orbital_years: f64 = 164.79132; }
