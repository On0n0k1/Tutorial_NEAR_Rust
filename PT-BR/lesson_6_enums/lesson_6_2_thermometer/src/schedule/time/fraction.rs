use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


/// Tipo que representa uma fração de segundo.
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Fraction(f32);


impl Fraction{
    pub fn new(fraction: f32) -> Self {
        assert!(fraction >= 0., "Error: Fraction of a second can not be negative. Value: {}.", fraction);
        assert!(fraction < 1., "Error: Fraction of a second must be between 0 and 1. Value: {}.", fraction);

        Fraction(fraction)
    }
}


impl From<f32> for Fraction {
    fn from(fraction: f32) -> Fraction {
        Fraction::new(fraction)
    }
}

impl From<&Fraction> for f32{
    fn from(fraction: &Fraction) -> f32 {
        let &Fraction(result) = fraction;

        result
    }
}

impl From<Fraction> for f32 {
    fn from(fraction: Fraction) -> f32 {
        f32::from(&fraction)
    }
}



