//! Date module
//! 
//! Used by timestamp
//! contains day, month and year
//! 

use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

pub mod day;
pub mod month;
pub mod year;

use day::Day;
use month::Month;
use year::Year;


/// Represents a Date (day, month, year).
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Date {
    day: Day,
    month: Month,
    year: Year,
}


impl Date{
    /// Creates a Date 
    /// 
    /// # Panics
    ///  - If day is negative.
    ///  - If day is higher than the max allowed for a particular month.
    ///  - If month is an invalid String.
    /// 
    pub fn new(day: u8, month: &str, year: i32) -> Date {
        // Creates a year 
        let year: Year = Year::new(year);
        // Creates a month
        let month: Month = Month::new(month);
        // Creates a day
        let day: Day = Day::new(day, &month, &year);

        Date{
            day,
            month,
            year,
        }
    }
}
