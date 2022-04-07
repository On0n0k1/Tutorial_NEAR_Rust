use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

/// Tipo que representa hora.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Hour(u8);


impl Hour{
    pub fn new(hour: u8) -> Hour {
        assert!(hour < 24, "Invalid value for hour. Must be lower than 24. Current: {}.", hour);

        Hour(hour)
    }
}


/// Nos permite usar u8::from(&nossoHour)
impl From<&Hour> for u8{
    fn from(hour: &Hour) -> u8 {
        let &Hour(result) = hour;

        result
    }
}


/// Nos permite usar u8::from(nossoHour)
impl From<Hour> for u8{
    fn from(hour: Hour) -> u8 {
        u8::from(&hour)
    }
}


/// Nos permite usar Hour::from(nossou8)
impl From<u8> for Hour{
    fn from(hour: u8) -> Hour {
        Hour::new(hour)
    }
}


