//! Minute module
//! 
//! This type is only an u8
//! 
//!  - u8::from(&minute) converts minute reference to u8 
//!  - u8::from(minute) converts minute to u8 
//!  - Minute::from(esteu8) converts u8 to Minute
//! 

use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


/// Represents a minute
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Minute(u8);


impl Minute {
    /// Builds an instance of Minute
    /// 
    /// # Panics
    /// If value is less than 60
    /// 
    pub fn new(minute: u8) -> Minute{
        assert!(minute < 60, "Invalid value for minute. Must be lower than 60. Current: {}.", minute);

        Minute(minute)
    }
}


/// Convert to u8 from &minute
impl From<&Minute> for u8 {
    fn from(minute: &Minute) -> u8 {
        let &Minute(result) = minute;

        result
    }
}


/// Convert to u8 from minute
impl From<Minute> for u8{
    fn from(minute: Minute) -> u8 {
        u8::from(&minute)
    }
}


/// Convert to minute from u8
impl From<u8> for Minute{
    fn from(minute: u8) -> Minute {
        Minute::new(minute)
    }
}
