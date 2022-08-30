//! Module with all functions related to a year
//! 
//! Year is represented as a tuple of an integer and string
//! 
use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


/// Tuple representing a year. 
/// 
/// An integer representing the year. 
/// A String representing a formatted year.
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Year(i32, String);


impl Year{
    /// create an instance of year
    pub fn new(mut value: i32) -> Year {
        let is_negative = value < 0;
        if is_negative{
            value = -value;
        }

        let text: &str = match is_negative{
            true => {
                "BC"
            },
            false => {
                "AD"
            }
        };

        let text: String = format!("{} {}", value, text);
        if is_negative {
            value = -value;
        }

        Year(value, text)
    }

    /// Returns year
    pub fn get(&self) -> i32 {
        // Year is a tuple, so using .0 is the first value.
        // i32 implements copy, so there's no need to self.0.clone()
        self.0
    }
}

/// Convert to String from &Year
impl From<&Year> for String{
    fn from(year: &Year) -> String {
        year.1.clone()
    }
}

/// Convert to String from Year
impl From<Year> for String{
    fn from(year: Year) -> String {
        String::from(&year)
    }
}

// Convert struct to String. Implementing this trait allows using Year in format!, println! and panic!
impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}


