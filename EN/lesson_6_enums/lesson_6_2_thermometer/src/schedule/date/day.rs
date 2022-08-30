//! Module with all functions related to a day
//! 
//! We use an u8 for the day, but we also need to 
//! check the day is valid. So, we'll need to make
//! day a struct Day(u8).
//! 
//! When serialized to JSON, the value would just be
//! an u8, so there won't be any additional complexity
//! for the user.
//! 
//! You can build Day using Day::new. 
//! This function needs both month and year to know
//! the max value for day and if it is a leap year.
//! Other features:
//!  - u8::from(day) allows converting a Day to u8.
//!  - String::from(day) allows converting a Day to String.
//!  - std::fmt::Display is implemented, and so it allows to 
//! use Daz in macros println! and panic!
//! 
//! ## Examples
//! 
//! ```rust
//! # use lesson_6_2_thermometer::schedule::date::day::Day;
//! # use lesson_6_2_thermometer::schedule::date::month::Month;
//! # use lesson_6_2_thermometer::schedule::date::year::Year;
//! 
//! // not leap year
//! let month = Month::new("feb");
//! let year = Year::new(1971);
//! 
//! let day = Day::new(28, &month, &year);
//! assert_eq!(u8::from(&day), 28);
//! assert_eq!(format!("{}", day), "28");
//! assert_eq!(String::from(&day), "28");
//! 
//! // leap year
//! let month = Month::new("feb");
//! let year = Year::new(1972);
//! 
//! let day = Day::new(29, &month, &year);
//! assert_eq!(u8::from(&day), 29);
//! assert_eq!(format!("{}", day), "29");
//! assert_eq!(String::from(&day), "29");
//! 
//! ```
//! 

use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Serialize, Deserialize },
};

use crate::schedule::date::{
    month::Month,
    year::Year,
};

/// Represents a day
/// 
/// When serialized, it will become an u8.
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Copy, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Day(u8);

impl Day{
    /// Creates an instance of Day.
    ///  - day: 1 to 31, depending on month and year.
    ///  - current_month: current month, for validation
    ///  - current_year: current year, for validation
    /// 
    /// # Panics
    /// - invalid day
    /// 
    pub fn new(day: u8, current_month: &Month, current_year: &Year) -> Self {
        let day = Day(day);
        day.assert_valid(current_month, current_year);

        day
    }

    /// # Panics
    /// - if day is invalid
    fn assert_valid(&self, current_month: &Month, current_year: &Year) {
        let &Day(day) = self;

        let mut current_year: i32 = current_year.get();

        // Se for negativo, converte para positivo
        if current_year < 0 {
            current_year = -current_year;
        }

        // true if "leap year".
        let leap_year: bool = (current_year % 4) == 0;
        // convert true = 1, false = 0.
        let leap_year: u8 = leap_year as u8;

        // source: https://www.rapidtables.com/calc/time/months-of-year.html
        let max_day: u8 = match current_month {
            &Month::January(_) => 31,
            &Month::February(_) => 28 + leap_year,
            &Month::March(_) => 31,
            &Month::April(_) => 30,
            &Month::May(_) => 31,
            &Month::June(_) => 30,
            &Month::July(_) => 31,
            &Month::August(_) => 31,
            &Month::September(_) => 30,
            &Month::October(_) => 31,
            &Month::November(_) => 30,
            &Month::December(_) => 31,
        };

        // check if day is within valid range
        assert!(day <= max_day,
            "Invalid values for day. Day: {}, Month: {}, Year: {}. Day for given month and year can not be higher than {}.",
                day,
                current_month,
                current_year,
                max_day,
        )
    }
}


/// Convert to u8 from Day
impl From<&Day> for u8{
    fn from(day: &Day) -> u8 {
        let &Day(result) = day;

        result
    }
}

/// Convert to String from Day
impl From<&Day> for String{
    fn from(day: &Day) -> String {
        u8::from(day).to_string()
    }
}

// Convert struct to String. Implementing this trait allows using Day in format!, println! and panic!
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
