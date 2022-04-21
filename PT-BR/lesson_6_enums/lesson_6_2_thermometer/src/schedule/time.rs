//! Módulo que representa tempo.
//! 
//! Usado por Schedule.
//! 
//! Composto por por Hour, Minute, Second.
//! 


use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


pub mod hour;
pub mod minute;
pub mod second;


use hour::Hour;
use minute::Minute;
use second::Second;

/// Representa tempo (hora, minuto, segundo).
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Time{
    hour: Hour,
    minute: Minute,
    second: Second,
}


impl Time {
    /// Constroi uma instância de Time.
    /// 
    /// # Panics
    ///  - se hora >= 24;
    ///  - se minute >= 60;
    ///  - se second >= 60. ;
    ///  - se second < 0. ;
    /// 
    pub fn new(hour: u8, minute: u8, second: f32) -> Time{
        let hour: Hour = Hour::new(hour);
        let minute: Minute = Minute::new(minute);
        let second: Second = Second::new(second);

        Time{
            hour,
            minute,
            second
        }
    }
}
