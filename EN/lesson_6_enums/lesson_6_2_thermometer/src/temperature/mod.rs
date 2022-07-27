//! Module: temperature
//! Has a temperature value (value) an a temperature unit (temp_format).
//! 
//! Smart Contract has a default temperature unit.
//! If you don't specify a temperature unit, we use the default system unit.
//! 
//! If the temperature unit sent in a message is different from the system-specified one, 
//! we convert it to the system temperature unit
//! 
pub mod temp_format;

use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

use temp_format::TemperatureUnit;
use crate::utils::log;

/// A Temperature.
/// temperature value is f32.
/// temperature unit can be Kelvin, Celsius or Fahrenheit.
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Temperature {
    degrees: f32,
    unit: TemperatureUnit,
}

impl Temperature {
    /// Create an instance of temperature
    /// 
    /// # Panic
    /// if temperature value is less than absolute 0
    /// 
    fn new_assert(temperature_value: f32, temperature_unit: TemperatureUnit) -> Self {
        let (min_allowed, name) = match &temperature_unit {
            TemperatureUnit::Celsius(unit_name) => {(-273.15, unit_name)},
            TemperatureUnit::Fahrenheit(unit_name) => {(-459.67, unit_name)},
            TemperatureUnit::Kelvin(unit_name) => {(0., unit_name)},
        };

        assert!(temperature_value >= min_allowed, "For temperature unit {}, temperature value can not be lower than {}. Temperature value passed: {}.", name, min_allowed, temperature_value);
        
        Temperature { 
            degrees: temperature_value, 
            unit: temperature_unit,
        }
    }

    /// Creates an instance of Temperature using the system temperature unit
    /// 
    /// "arg_temp" é o formato de temperatura da mensagem recebida.
    /// 
    /// Se arg_temp for diferente de temp_format. Seguirá estes passos:
    ///  - Cria uma instância de temperatura no formato de arg_temp;
    ///  - Converte seu formato para temp_format;
    ///  - Retorna a temperatura;
    /// 
    pub fn new(temperature_value: f32, temperature_unit: &TemperatureUnit, arg_temp: Option<String>) -> Self {
        match arg_temp {
            None => {
                Temperature::new_assert(temperature_value, temperature_unit.clone())
            },
            Some(arg_string) => {
                let mut arg_temp = Temperature::new_assert(temperature_value, TemperatureUnit::new(&arg_string));

                // Convert temperature reading into system temperature unit
                arg_temp.convert(temperature_unit);
                arg_temp
            }
        }

    }

    /// Atualiza temperatura se o formato for diferente. 
    /// 
    /// Retorna true se houver mudança.
    /// 
    pub fn update_temp_format(&mut self, temperature_unit: &TemperatureUnit) -> bool {
        let comparison = self.unit == *temperature_unit;

        if !comparison {
            self.convert(temperature_unit);
        }

        !comparison
    }
    
    /// Convert temperature units
    pub fn convert(&mut self, temperature_unit: &TemperatureUnit){
        let current_unit: TemperatureUnit = self.unit.clone();
        let current_value = self.degrees;
        
        log(&format!("Converting temperature to system format. System Unit: {}, Current Unit: {}.", temperature_unit, &current_unit));

        match (current_unit, temperature_unit) {
            (TemperatureUnit::Kelvin(_), &TemperatureUnit::Celsius(_)) => {
                // C = K − 273.15
                self.degrees = current_value - 273.15;    
                self.unit = TemperatureUnit::new("c");
            },
            (TemperatureUnit::Kelvin(_), &TemperatureUnit::Fahrenheit(_)) => {
                // F = (K – 273.15) × 9⁄5 + 32
                self.degrees = (current_value - 273.15) * 9.0 / 5.0 + 32.0;
                self.unit = TemperatureUnit::new("f");
            },
            (TemperatureUnit::Celsius(_), &TemperatureUnit::Kelvin(_)) => {
                // K = C + 273.15
                self.degrees = current_value + 273.15;
                self.unit = TemperatureUnit::new("k");
            },
            (TemperatureUnit::Celsius(_), &TemperatureUnit::Fahrenheit(_)) => {
                // F = C(9⁄5) + 32
                self.degrees = current_value * (9.0 / 5.0) + 32.0;
                self.unit = TemperatureUnit::new("f");
            },
            (TemperatureUnit::Fahrenheit(_), &TemperatureUnit::Kelvin(_)) => {
                // K = (F − 32) × 5⁄9 + 273.15
                self.degrees = (current_value - 32.0) * 5.0 / 9.0 + 273.15;
                self.unit = TemperatureUnit::new("k");
            },
            (TemperatureUnit::Fahrenheit(_), &TemperatureUnit::Celsius(_)) => {
                // C = (F − 32) × 5⁄9
                self.degrees = (current_value - 32.0) * 5.0 / 9.0;
                self.unit = TemperatureUnit::new("c");
            },
            (_, _) => {
                // Todas alternativas diferentes foram consideradas. Isso considera todas as situações em que os tipos são iguais.
                // Portanto, não fazemos nada.
                return;
            }
        }
    }
}
