use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
enum TemperatureUnits {
    Celsius,
    Farenheit,
    Kelvin,
}
#[derive(Debug, Clone, Copy)]
pub struct Temperature {
    /// Temperature units are internally represented as degrees celsius
    base_value: f64,
    units: TemperatureUnits,
}
impl Display for Temperature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (value, units) = match self.units {
            TemperatureUnits::Celsius => (self.base_value, "°C"),
            TemperatureUnits::Farenheit => (self.base_value*1.8 + 32., "°F"),
            TemperatureUnits::Kelvin => (self.base_value + 273.15, "K")
        };

        write!(f, "{:.2}{}", value, units)
    }
}
impl PartialEq for Temperature {
    fn eq(&self, other: &Self) -> bool {
        self.base_value.round() == other.base_value.round()
    }
}
impl Temperature {
    pub fn as_celsius(&self) -> Self {
        Self {base_value: self.base_value, units: TemperatureUnits::Celsius}
    }
    pub fn as_farenheit(&self) -> Self {
        Self {base_value: self.base_value, units: TemperatureUnits::Farenheit}
    }
    pub fn as_kelvin(&self) -> Self {
        Self {base_value: self.base_value, units: TemperatureUnits::Kelvin}
    }
}

pub trait IntoTemperature { 
    fn to_celsius(&self) -> Temperature;
    fn to_farenheit(&self) -> Temperature;
    fn to_kelvin(&self) -> Temperature;
}

impl IntoTemperature for f64 {
    fn to_celsius(&self)   -> Temperature { Temperature { base_value: *self, units: TemperatureUnits::Celsius }}
    fn to_farenheit(&self) -> Temperature { Temperature { base_value: (*self-32.)/1.8, units: TemperatureUnits::Farenheit }}
    fn to_kelvin(&self)    -> Temperature { Temperature { base_value: (*self-273.15), units: TemperatureUnits::Kelvin }}
}
impl From<Temperature> for f64 {
    fn from(temp: Temperature) -> Self {
        match temp.units {
            TemperatureUnits::Celsius => temp.base_value,
            TemperatureUnits::Farenheit => temp.base_value *1.8 + 32.,
            TemperatureUnits::Kelvin => temp.base_value + 273.15,
        }
    }
}
impl IntoTemperature for f32 {
    fn to_celsius(&self) -> Temperature {(*self as f64).to_celsius()}
    fn to_farenheit(&self) -> Temperature {(*self as f64).to_farenheit()}
    fn to_kelvin(&self) -> Temperature {(*self as f64).to_kelvin()}
}
impl From<Temperature> for f32 {
    fn from(temp: Temperature) -> Self {
        let f: f64 = temp.into();
        f as f32
    }
}



#[cfg(test)]
mod temp_tests {
    use super::*;

    #[test]
    pub fn equality() {
        println!("{}", 54.0.to_farenheit());

        assert_eq!((0.0).to_celsius(), (32.0).to_farenheit());
        assert_eq!((0.0).to_kelvin(), (-273.15).to_celsius());
        assert_eq!((32.0).to_farenheit(), (273.15).to_kelvin());
    }
}