//! Temperature Unit module
//! 
//! Unit can be Kelvin, Celsius or Fahrenheit.
//! The default is to use Kelvin.
//! 
//! Implemented TraitsÖ
//!  - Default. Default unit is Celsius::Kelvin.
//!  - PartialEq and Eq. Allows comparing between Units.
//!  - String::from(&temperature_unit) converts a &temperature_unit reference to String;
//!  - String::from(temperature_unit) converts temperature_unit to String;
//!  - TemperatureUnit::from("a str") converts an &str to TemperatureUnit;
//!  - TemperatureUnit::from(aString) converts a String to TemperatureUnit;
//!  - TemperatureUnit::from(&aString) converts a &String reference to TemperatureUnit;
//!  - std::fmt::Display. Allows display of value using macros like println!, format! e panic!;
//! 

use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


/// Represents a TemperatureUnit (Kelvin, Celsius and Fahrenheit).
/// 
/// Temperature unit management, as we can have multiple sensors using different temperature units.
/// This guarantees all possibilities are in sync and correct.
/// 
/// This enum is seen a String when ser/deserialzing JSON.
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum TemperatureUnit {
    Celsius(String),
    Fahrenheit(String),
    Kelvin(String),
}


impl TemperatureUnit {
    /// Creates a TemperatureUnit
    /// 
    /// Not case-sensitive. You can specify temperature units like:
    /// 
    ///  - "celsius", "c" => TempFormat::Celsius("Celsius")
    ///  - "fahrenheit", "f" => TempFormat::Fahrenheit("Fahrenheit")
    ///  - "kelvin", "k" => TempFormat::Kelvin("Kelvin")
    /// 
    /// # Panics
    /// - If unit name is invalid.
    /// 
    pub fn new(unit_name: &str) -> Self{
        // This conversion for &str to TemperatureUnit is possible due to From<&str> being implemented
        let lower_case: String = unit_name.to_ascii_lowercase();

        // let's return what matches OR panic!
        match &lower_case[..] {
            "celsius" | "c" => TemperatureUnit::Celsius(String::from("Celsius")),
            "fahrenheit" | "f" => TemperatureUnit::Fahrenheit(String::from("Fahrenheit")),
            "kelvin" | "k" => TemperatureUnit::Kelvin(String::from("Kelvin")),
            invalid_name => panic!("Invalid temperature unit name ({}). Valid args: ['Celsius', 'c', 'Fahrenheit', 'f', 'Kelvin', 'k']", invalid_name),
        }
    }
}


/// The default unit will set to Kelvin, but 
/// this can later be changed after contract initialization
/// 
impl Default for TemperatureUnit {
    fn default() -> Self {
        TemperatureUnit::new("k")
    }
}

// Allows partial comparison between temperate units
//
// A = B doesn't mean B = A
// A = B and A = C doesn't mean B = C
//
impl PartialEq for TemperatureUnit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TemperatureUnit::Celsius(_), TemperatureUnit::Celsius(_)) => true,
            (TemperatureUnit::Fahrenheit(_), TemperatureUnit::Fahrenheit(_)) => true,
            (TemperatureUnit::Kelvin(_), TemperatureUnit::Kelvin(_)) => true,
            (_, _) => false,
        }
    }
}

// This trait allows total comparison between temperature units
//
// A = B guarantees B = A
//
// A = B and A = C guarantees B = C
//
impl Eq for TemperatureUnit {}


/// Conversion to String fro &TemperatureUnit
impl From<&TemperatureUnit> for String{
    fn from(temperature_unit: &TemperatureUnit) -> String {
        match &temperature_unit {
            TemperatureUnit::Celsius(value) => (*value).clone(),
            TemperatureUnit::Kelvin(value) => (*value).clone(),
            TemperatureUnit::Fahrenheit(value) => (*value).clone(),
        }
    }
}

/// Conversion to String from TemperatureUnit
impl From<TemperatureUnit> for String {
    fn from(temperature_unit: TemperatureUnit) -> String {
        String::from(&temperature_unit)
    }
}


/// Conversion to TemperatureUnit from &str
impl From<&str> for TemperatureUnit{
    fn from(temperature_unit_name: &str) -> TemperatureUnit {
        TemperatureUnit::new(temperature_unit_name)
    }
}


/// Conversion to TemperatureUnit from &String
impl From<&String> for TemperatureUnit{
    fn from(temperature_unit_name: &String) -> TemperatureUnit {
        TemperatureUnit::from(&temperature_unit_name[..])
    }
}


/// Conversion to TemperatureUnit from String
impl From<String> for TemperatureUnit{
    fn from(temperature_unit_name: String) -> TemperatureUnit{
        TemperatureUnit::from(&temperature_unit_name[..])
    }
}


/// Allos displaying the enum as a String and is used by macros such as format!, println! and panic!.
impl std::fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
