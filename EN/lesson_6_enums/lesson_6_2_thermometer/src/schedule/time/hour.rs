//! Hour module
//! 
//! This type is only an u8
//! 
//!  - u8::from(&hour) converts hour reference to u8 
//!  - u8::from(hour) converts hour value to u8
//!  - Hour::from(value u8) converts u8 value to Hour
//! 

use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

/// A type represening an Hour
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Hour(u8);

impl Hour{
    pub fn new(hour: u8) -> Hour {
        assert!(hour < 24, "Invalid value for hour. Must be lower than 24. Current: {}.", hour);

        Hour(hour)
    }
}

/// Convert to u8 from &Hour
impl From<&Hour> for u8{
    fn from(hour: &Hour) -> u8 {
        let &Hour(result) = hour;

        result
    }
}

/// Convert to u8 from Hour
impl From<Hour> for u8{
    fn from(hour: Hour) -> u8 {
        u8::from(&hour)
    }
}

/// Convert to Hour from u8
impl From<u8> for Hour{
    fn from(hour: u8) -> Hour {
        Hour::new(hour)
    }
}
