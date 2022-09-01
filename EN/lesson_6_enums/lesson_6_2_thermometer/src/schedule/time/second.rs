//! Second module
//! 
//! This type is only an f32
//! 
//!  - f32::from(&second) converts minute reference to f32.
//!  - f32::from(second) converts minute to f32.
//!  - Minute::from(value f32) converts f32 to Second
//! 

use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

/// Represents a second
/// # Panics
///  - If value is higher than 60.
///  - If value is negative.
/// 
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


/// Convert to f32 from &Second
impl From<&Second> for f32 {
    fn from(second: &Second) -> f32 {
        let &Second(result) = second;

        result
    }
}


/// Convert to f32 from Second
impl From<Second> for f32{
    fn from(second: Second) -> f32 {
        f32::from(&second)
    }
}


/// Convert to Second from f32
impl From<f32> for Second{
    fn from(second: f32) -> Second {
        Second::new(second)
    }
}
