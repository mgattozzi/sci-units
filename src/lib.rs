use std::ops::Add;
use std::fmt;
use Temperature::*;

#[derive(Debug, PartialEq, Copy, Clone)]
/// An enum representing the different units of Temperature
pub enum Temperature {
    Kelvin(f64),
    Celsius(f64),
    Fahrenheit(f64),
}

impl fmt::Display for Temperature {
    fn fmt(&self, fmtr: &mut fmt::Formatter) -> fmt::Result {
        match *self {
             Kelvin(k) => write!(fmtr, "{}K", k),
             Celsius(c) => write!(fmtr, "{}°C", c),
             Fahrenheit(f) => write!(fmtr, "{}°F", f),
        }
    }
}

impl Temperature {
    /// Convert whatever Temperature unit there is into Celsius
    pub fn to_celsius(self) -> Temperature {
        match self {
            Kelvin(k) => Celsius(k - 273.15),
            c @ Celsius(_) => c,
            Fahrenheit(f) =>Celsius( (f-32.0) * (5.0/9.0) ),
        }
    }

    /// Convert whatever Temperature unit there is into Fahrenheit
    pub fn to_fahrenheit(self) -> Temperature {
        match self {
            Kelvin(k) =>  Fahrenheit( (k * (9.0/5.0)) - 459.67 ),
            Celsius(c) => Fahrenheit( (c * (9.0/5.0)) + 32.0 ),
            f @ Fahrenheit(_) => f,
        }
    }

    /// Convert whatever Temperature unit there is into Kelvin
    pub fn to_kelvin(self) -> Temperature {
        match self {
            k @ Kelvin(_) => k,
            Celsius(c) => Kelvin(c + 273.15),
            Fahrenheit(f) => Kelvin( (f + 459.67) * (5.0/9.0) ),
        }
    }
}

impl Add for Temperature {

    type Output = Temperature;

    /// Add the Temperature units together with automatic conversion.
    /// The RHS will be converted into the unit on the left.
    fn add(self, rhs: Temperature) -> Self::Output {
        match (self, rhs) {
            (Celsius(a), b @ _) => {
                match b.to_celsius() {
                    Celsius(b) => Celsius(a + b),
                    _ => unreachable!(),
                }
            },
            (Fahrenheit(a), b @ _) => {
                match b.to_fahrenheit() {
                    Fahrenheit(b) => Fahrenheit(a + b),
                    _ => unreachable!(),
                }
            },
            (Kelvin(a), b @ _) => {
                match b.to_kelvin() {
                    Kelvin(b) => Kelvin(a + b),
                    _ => unreachable!(),
                }
            },
        }
    }
}

#[test]
fn add_test() {
    let k1 = Kelvin(0.0);
    let k2 = Kelvin(100.0);

    let c1 = Celsius(0.0);
    let c2 = Celsius(100.0);

    let f1 = Fahrenheit(0.0);
    let f2 = Fahrenheit(100.0);

    // Added to itself it should be the same unit
    assert_eq!(Kelvin(100.0), k1 + k2);
    assert_eq!(Celsius(100.0), c1 + c2);
    assert_eq!(Fahrenheit(100.0), f1 + f2);

    // Added to another unit it should be the conversion of the right
    // into the unit on the left added together. Remember we are using
    // floating point so there will be some margin of error for fractions
    // that occur
    assert_eq!(Kelvin(273.15), k1 + c1);
    assert_eq!(Kelvin(255.3722222222222223), k1 + f1);
    assert_eq!(Celsius(-273.15), c1 + k1);
    assert_eq!(Celsius(-17.77777777777778), c1 + f1);
    assert_eq!(Fahrenheit(32.0), f1 + c1);
    assert_eq!(Fahrenheit(-459.67), f1 + k1);

    // Testing multiple unit types added together
    assert_eq!(Fahrenheit(-427.67), f1 + k1 + c1);
    assert_eq!(Celsius(-290.92777777777775), c1 + k1 + f1);
    assert_eq!(Kelvin(528.5222222222222), k1 + f1 + c1);
}

#[test]
fn format_test() {
    assert_eq!(format!("{}", Kelvin(528.0)), "528K".to_owned());
    assert_eq!(format!("{}", Celsius(100.0)), "100°C".to_owned());
    assert_eq!(format!("{}", Fahrenheit(32.0)), "32°F".to_owned());
}
