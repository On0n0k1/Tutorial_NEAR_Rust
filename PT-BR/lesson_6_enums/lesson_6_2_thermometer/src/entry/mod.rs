use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


use crate::{
    schedule::Schedule,
    temperature::{
        Temperature,
        temp_format::TempFormat,
    }
};


/// Representa um momento de temperatura e tempo.
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Entry{
    schedule: Schedule,
    temperature: Temperature,
}


impl Entry{
    /// Constroi uma instância de Entry.
    /// 
    ///  - Se time (horario) for omitido. O valor será o momento da chamada da mensagem.
    ///  - Se date (data) for omitida. O valor será o dia da chamada da mensagem.
    ///  - Se o formato de temperatura for omitido, utilizará o formato de temperatura do sistema.
    ///  - value representa o valor de temperatura.
    /// 
    pub fn new(
            time: Option<(u8, u8, f32)>,
            date: Option<(i32, String, u8)>,
            temp_format: &TempFormat, 
            value: f32, 
            arg_temp: Option<String>,
        ) -> Self {
        
        let schedule: Schedule = Schedule::new(date, time);
        let temperature: Temperature = Temperature::new(value, temp_format, arg_temp);

        Entry { 
            schedule, 
            temperature,
        }
    }

    /// Atualiza o formato e valor de temperatura da entry.
    pub fn update_temp_format(&mut self, new_format: &TempFormat) -> bool{
        self.temperature.update_temp_format(new_format)
    }
}


