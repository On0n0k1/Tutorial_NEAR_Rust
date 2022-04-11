use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


// Fração de segundo.
pub mod fraction;
pub mod hour;
pub mod minute;
pub mod second;


use fraction::Fraction;
use hour::Hour;
use minute::Minute;
use second::Second;


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Time{
    fraction: Fraction,
    hour: Hour,
    minute: Minute,
    second: Second,
}


impl Time {
    pub fn new(hour: u8, minute: u8, second: u8, fraction: f32) -> Time{
        let hour: Hour = Hour::new(hour);
        let minute: Minute = Minute::new(minute);
        let second: Second = Second::new(second);
        let fraction: Fraction = Fraction::new(fraction);

        Time{
            fraction,
            hour,
            minute,
            second
        }
    }
}
