use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Second(u8);

impl Second {
    pub fn new(second: u8) -> Second{
        assert!(second < 60, "Invalid value for second. Must be lower than 60. Current: {}.", second);

        Second(second)
    }
}


/// Nos permite usar u8::from(&nossoSecond)
impl From<&Second> for u8{
    fn from(second: &Second) -> u8 {
        let &Second(result) = second;

        result
    }
}


/// Nos permite usar u8::from(nossoSecond)
impl From<Second> for u8{
    fn from(second: Second) -> u8 {
        u8::from(&second)
    }
}


/// Nos permite usar Second::from(nossou8)
impl From<u8> for Second{
    fn from(second: u8) -> Second {
        Second::new(second)
    }
}
