use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

pub mod date;
pub mod time;

use date::Date;
use time::Time;

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Schedule{
    date: Date,
    time: Time,
}


impl Schedule{
    pub fn new(year: i32, month: &str, day: u8, hour: u8, minute: u8, second: u8, fraction: f32) -> Self {
        let date: Date = Date::new(day, month, year);
        let time: Time = Time::new(hour, minute, second, fraction);

        Schedule{
            date,
            time,
        }
    }
}

