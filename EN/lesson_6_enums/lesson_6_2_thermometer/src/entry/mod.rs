//! Entry represents a single input value from a user
//! 
//! Periodicamente cada sensor enviará o valor de temperatura coletado. O contrato armazenará cada entry associada ao tempo recebido.
//! 
//! Cada Entry possui os seguintes atributos:
//!  - temperature: Um valor de temperatura (f32), associado a um formato de temperatura (Kelvin, Celsius, Fahrenheit);
//!  - schedule: Momento de recebimento do input. Formato de tempo é UTC. Que tal implementar suporte a diversos fuso-horarios depois?
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


/// Representa um momento de temperatura e tempo.
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct TemperatureReading {
    timestamp: Timestamp,
    measurement: Temperature,
}


impl TemperatureReading {
    /// Constroi uma instância de Entry.
    /// 
    ///  - Se time (horario) for omitido. O valor será o momento da chamada da mensagem.
    ///  - Se date (data) for omitida. O valor será o dia da chamada da mensagem.
    ///  - Se o formato de temperatura for omitido, utilizará o formato de temperatura do sistema.
    ///  - value representa o valor de temperatura.
    /// 
    ///  # Panics
    ///  - se temperatura for menor que zero absoluto;
    ///  - se dia for inválido;
    ///  - se mês for inválido;
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

    /// Se new_format for um formato de temperatura diferente do atual. Atualiza e realiza a conversão de valores.
    pub fn update_temp_format(&mut self, new_format: &TemperatureUnit) -> bool{
        self.measurement.update_temp_format(new_format)
    }
}


