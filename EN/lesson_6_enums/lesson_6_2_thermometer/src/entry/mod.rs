//! Entry represents a single input value from a user
//! 
//! Periodically each sensor will send temperature readings. The Smart Contract will save these as it receives them.
//! 
//! Each temperature reading has the following attributes: 
//!  - temperature: a temperature value (f32) with a temperature unit (Kelvin, Celsius, Fahrenheit).
//!  - schedule: a timestamp when the the measurement was taken. UTC.
//! 

use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


use crate::{
    schedule::Timestamp,
    temperature::{
        Temperature,
        temp_format::TemperatureUnit,
    }
};


/// Represents a temperature reading 
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TemperatureReading {
    timestamp: Timestamp,
    measurement: Temperature,
}


impl TemperatureReading {
    /// Creates a temperature reading
    /// 
    ///  - If time is omitted, the call time will be used as default.
    ///  - Id date is omitted, the call date will be used as default.
    ///  - If temperature unit is ommited, the system's default will be used as default.
    ///  - Value represents the actual temperature value/measurement.
    /// 
    ///  # Panics
    ///  - if temperature below absolute zero.
    ///  - On invalid day.
    ///  - On invalid month.
    /// 
    pub fn new(
            time: Option<(u8, u8, f32)>,
            date: Option<(i32, String, u8)>,
            temperature_unit: &TemperatureUnit, 
            temperature_value: f32, 
            arg_temp: Option<String>,
        ) -> Self {
        
        TemperatureReading { 
            timestamp: Timestamp::new(date, time), 
            measurement: Temperature::new(temperature_value, temperature_unit, arg_temp),
        }
    }

    /// If there's a new temperature unit given, perform conversion
    pub fn update_temp_format(&mut self, new_format: &TemperatureUnit) -> bool {
        self.measurement.update_temp_format(new_format)
    }
}


