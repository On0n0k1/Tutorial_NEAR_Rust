use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Minute(u8);


impl Minute {
    pub fn new(minute: u8) -> Minute{
        assert!(minute < 60, "Invalid value for minute. Must be lower than 60. Current: {}.", minute);

        Minute(minute)
    }
}


/// Nos permite usar u8::from(&nossoMinute)
impl From<&Minute> for u8{
    fn from(minute: &Minute) -> u8 {
        let &Minute(result) = minute;

        result
    }
}


/// Nos permite usar u8::from(nossoMinute)
impl From<Minute> for u8{
    fn from(minute: Minute) -> u8 {
        u8::from(&minute)
    }
}


/// Nos permite usar Minute::from(nossou8)
impl From<u8> for Minute{
    fn from(minute: u8) -> Minute {
        Minute::new(minute)
    }
}
