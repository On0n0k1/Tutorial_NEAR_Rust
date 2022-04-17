use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Second(f32);

impl Second {
    pub fn new(second: f32) -> Second{
        assert!(second < 60., "Invalid value for second. Must be lower than 60. Current: {}.", second);
        assert!(second >= 0., "Invalid value for second. Can't be negative. Current: {}.", second);

        Second(second)
    }
}


/// Nos permite usar f32::from(&nossoSecond)
impl From<&Second> for f32{
    fn from(second: &Second) -> f32 {
        let &Second(result) = second;

        result
    }
}


/// Nos permite usar f32::from(nossoSecond)
impl From<Second> for f32{
    fn from(second: Second) -> f32 {
        f32::from(&second)
    }
}


/// Nos permite usar Second::from(nossof32)
impl From<f32> for Second{
    fn from(second: f32) -> Second {
        Second::new(second)
    }
}
