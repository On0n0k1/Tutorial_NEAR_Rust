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


#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Entry{
    schedule: Schedule,
    temperature: Temperature,
}


impl Entry{
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

    pub fn update_temp_format(&mut self, new_format: &TempFormat) -> bool{
        self.temperature.update_temp_format(new_format)
    }
}


