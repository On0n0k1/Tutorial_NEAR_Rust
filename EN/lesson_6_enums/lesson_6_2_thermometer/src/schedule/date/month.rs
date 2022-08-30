//! Module with all functions related to a month
//! 
//! We'll use an enum with all possible month value
//! For JSON, it is better to use a string or a number.
//! 
//! Using serde, we can choose the best option. 
//! Let's first declare our enum.
//! 
//! ```
//!use near_sdk::{
//!    borsh::{ self, BorshDeserialize, BorshSerialize },
//!    serde::{ Deserialize, Serialize },
//!};
//! 
//! #[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
//! #[serde(crate = "near_sdk::serde")]
//! pub enum Month{
//!     January(String),
//!     February(String),
//!     March(String),
//!     April(String),
//!     May(String),
//!     June(String),
//!     July(String),
//!     August(String),
//!     September(String),
//!     October(String),
//!     November(String),
//!     December(String),
//! }
//! ```
//! 
//! If month is Month::December(String::from("December")), 
//! then our value JSON will be {December: "December"}.
//! 
//! But, if we use untagged, 
//! serde(untagged), 
//! 
//! ```
//!use near_sdk::{
//!    borsh::{ self, BorshDeserialize, BorshSerialize },
//!    serde::{ Deserialize, Serialize },
//!};
//! 
//! #[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
//! #[serde(crate = "near_sdk::serde")]
//! #[serde(untagged)]
//! pub enum Month{
//!     January(String),
//!     February(String),
//!     March(String),
//!     April(String),
//!     May(String),
//!     June(String),
//!     July(String),
//!     August(String),
//!     September(String),
//!     October(String),
//!     November(String),
//!     December(String),
//! }
//! ```
//! 
//! Then no tag will be used and so 
//! Month::december(String::from("December")), 
//! will be represented as month: "December". 
//! which is more user-friendly.
//! 

use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


/// Represents a month
/// 
/// Using serde(untagged) this enum will
/// be represented as string (no tag)
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum Month{
    January(String),
    February(String),
    March(String),
    April(String),
    May(String),
    June(String),
    July(String),
    August(String),
    September(String),
    October(String),
    November(String),
    December(String),
}


impl Month{
    /// Create a month instance.
    /// 
    /// All possible values on the left are converted
    /// to an enum value on the right:
    /// 
    ///  - "january", "jan", "janeiro", "enero", "ene" => Month::January("January")
    ///  - "february", "feb", "fevereiro", "fev", "febrero" => Month::February("February")
    ///  - "march", "mar", "março", "marzo" => Month::March("March")
    ///  - "april", "apr", "abril", "abr" => Month::April("April")
    ///  - "may", "maio", "mayo" => Month::May("May")
    ///  - "june", "jun", "junho", "junio" => Month::June("June")
    ///  - "july", "jul", "julho", "julio" => Month::July("July")
    ///  - "august", "aug", "agosto", "ago" => Month::August("August")
    ///  - "september", "sep", "setembro", "set", "septiembre" => Month::September("September")
    ///  - "october", "octo", "oct", "outubro", "out", "octubre", "octu" => Month::October("October")
    ///  - "november", "nov", "novembro", "noviembre" => Month::November("November")
    ///  - "december", "dec", "dezembro", "dez", "diciembro", "dic" => Month::December("December")
    /// 
    /// # Panics
    /// - if an invalid argument is provided. Month not valid.
    /// 
    pub fn new(month: &str) -> Self {
        let lower_case: String = month.to_ascii_lowercase();
        
        match &lower_case[..]{
            "january" | "jan" | "janeiro" | "enero" | "ene" => Month::January(String::from("January")),
            "february" | "feb" | "fevereiro" | "fev" | "febrero" => Month::February(String::from("February")),
            "march" | "mar" | "março" | "marzo" => Month::March(String::from("March")),
            "april" | "apr" | "abril" | "abr" => Month::April(String::from("April")),
            "may" | "maio" | "mayo" => Month::May(String::from("May")),
            "june" | "jun" | "junho" | "junio" => Month::June(String::from("June")),
            "july" | "jul" | "julho" | "julio" => Month::July(String::from("July")),
            "august" | "aug" | "agosto" | "ago" => Month::August(String::from("August")),
            "september" | "sep" | "setembro" | "set" | "septiembre" => Month::September(String::from("September")),
            "october" | "octo" | "oct" | "outubro" | "out" | "octubre" | "octu" => Month::October(String::from("October")),
            "november" | "nov" | "novembro" | "noviembre" => Month::November(String::from("November")),
            "december" | "dec" | "dezembro" | "dez" | "diciembre" | "dic" => Month::December(String::from("December")),
            invalid => panic!("Invalid value for month: {}.", invalid),
        }
    }

    // jan 31
    // feb 59
    // mar 90
    // apr 120
    // may 151
    // jun 181
    // jul 212
    // aug 243
    // sep 273
    // octo 304
    // nov 334
    // dec 365

    /// Value in range 0 and 365. 
    /// Return month and day, based on year
    /// 
    /// if is_leap_year then the range can be 0 to 366.
    /// 
    pub fn new_from_days(mut days: u64, is_leap_year: bool) -> (String, u8) {
        // 0 false 1 true
        let leap_year: u64 =  is_leap_year as u64;
        assert!(days < 365 + leap_year, "Unexpected Behavior. Days should be lower than {}. Got {}.", 365 + leap_year, days);
        // day 0 = jan 1
        days += 1;

        if days <= 31 {
            return (String::from("jan"), days as u8);
        }
        if days <= 59 + leap_year {
            // include feb 29 for leap year
            return (String::from("feb"), days as u8 - 31);
        }
        if days <= 90 + leap_year {
            return (String::from("mar"), (days - 59 - leap_year) as u8);
        }
        if days <= 120 + leap_year {
            return (String::from("apr"), (days - 90 - leap_year) as u8);
        }
        if days <= 151 + leap_year {
            return (String::from("may"), (days - 120 - leap_year) as u8);
        }
        if days <= 181 + leap_year {
            return (String::from("jun"), (days - 151 - leap_year) as u8);
        }
        if days <= 212 + leap_year {
            return (String::from("jul"), (days - 181 - leap_year) as u8);
        }
        if days <= 243 + leap_year {
            return (String::from("aug"), (days - 212 - leap_year) as u8);
        }
        if days <= 273 + leap_year {
            return (String::from("sep"), (days - 243 - leap_year) as u8);
        }
        if days <= 304 + leap_year {
            return (String::from("oct"), (days - 273 - leap_year) as u8);
        }
        if days <= 334 + leap_year {
            return (String::from("nov"), (days - 304 - leap_year) as u8);
        }

        (String::from("dec"), (days - 334 - leap_year) as u8)  
    }

    /// Returns month name as a String
    pub fn get(&self) -> String {
        match self {
            Month::January(value) => value.clone(),
            Month::February(value) => value.clone(),
            Month::March(value) => value.clone(),
            Month::April(value) => value.clone(),
            Month::May(value) => value.clone(),
            Month::June(value) => value.clone(),
            Month::July(value) => value.clone(),
            Month::August(value) => value.clone(),
            Month::September(value) => value.clone(),
            Month::October(value) => value.clone(),
            Month::November(value) => value.clone(),
            Month::December(value) => value.clone(),
        }
    }
}

/// Convert to u8 from month
impl From<&Month> for u8 {
    fn from(month: &Month) -> u8 {
        match month {
            Month::January(_) => 0,
            Month::February(_) => 1,
            Month::March(_) => 2,
            Month::April(_) => 3,
            Month::May(_) => 4,
            Month::June(_) => 5,
            Month::July(_) => 6,
            Month::August(_) => 7,
            Month::September(_) => 8,
            Month::October(_) => 9,
            Month::November(_) => 10,
            Month::December(_) => 11,
        }
    }
}

/// Convert to Month from u8
impl From<u8> for Month {
    fn from(month: u8) -> Month {
        match month{
            0 => Month::new("jan"),
            1 => Month::new("feb"),
            2 => Month::new("mar"),
            3 => Month::new("apr"),
            4 => Month::new("may"),
            5 => Month::new("jun"),
            6 => Month::new("jul"),
            7 => Month::new("aug"),
            8 => Month::new("sep"),
            9 => Month::new("oct"),
            10 => Month::new("nov"),
            11 => Month::new("dec"),
            invalid => panic!("Invalid value for month: {}. Number value must be positive, lower than 12.", invalid),
        }
    }
}

/// Convert to String from &Month
impl From<&Month> for String{
    fn from(month: &Month) -> String {
        month.get()
    }
}


/// Convert to String from Month
impl From<Month> for String{
    fn from(month: Month) -> String {
        String::from(&month)
    }
}

/// String representation, useful for using format!, println! and panic!
impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}




#[cfg(test)]
mod tests{
    use crate::utils::log;
    use crate::schedule::Month;


    #[test]
    /// Test Month::new_from_days 
    /// for all days of year and leap year 
    fn new_from_days(){

        /// testing year. parameters:
        ///  - days: range 0 to 365. 0 to 366 if is_leap_year = true.
        ///  - is_leap_year: true if leap year
        ///  - expected_month: range 0 to 12.
        ///  - expected_day: range 0 to 31, depending on month and if leap year
        /// 
        fn util_day_constructor(days: u64, is_leap_year: bool, expected_month: u8, expected_day: u8) {
            let (month_str, day) = Month::new_from_days(days, is_leap_year);
            let month_number = u8::from(&Month::new(&month_str));
    
            assert_eq!(
                month_number, 
                expected_month, 
                "Comparison error when constructing day. Expected month: {}, number: {}. Got {}.", 
                Month::from(expected_month), 
                expected_month, 
                month_number,
            );

            assert_eq!(
                day, 
                expected_day, 
                "Comparison error when constructing day. Expected day: {}. Got: {}.", 
                expected_day, 
                day,
            );
        }

        /// Use util_day_constructor to test all days of the year
        fn testing_year(is_leap_year: bool){
            let leap_year = is_leap_year as u64;

            #[allow(unused_mut)]
            let mut day_number;
            #[allow(unused_mut)]
            let mut month_number;

            for day in 0..(365 + leap_year) {
                if day < 31 { 
                    day_number = day;
                    month_number = 0; 
                } else if day < 59 + leap_year{ 
                    day_number = day - 31;
                    month_number = 1; 
                } else if day < 90 + leap_year { 
                    day_number = day - 59 - leap_year;
                    month_number = 2;  
                } else if day < 120 + leap_year{ 
                    day_number = day - 90 - leap_year;
                    month_number = 3; 
                } else if day < 151 + leap_year{ 
                    day_number = day - 120 - leap_year;
                    month_number = 4; 
                } else if day < 181 + leap_year { 
                    day_number = day - 151 - leap_year;
                    month_number = 5; 
                } else if day < 212 + leap_year{ 
                    day_number = day - 181 - leap_year;
                    month_number = 6; 
                } else if day < 243 + leap_year{ 
                    day_number = day - 212 - leap_year;
                    month_number = 7; 
                } else if day < 273 + leap_year{ 
                    day_number = day - 243 - leap_year;
                    month_number = 8; 
                } else if day < 304 + leap_year{ 
                    day_number = day - 273 - leap_year;
                    month_number = 9; 
                } else if day < 334 + leap_year{ 
                    day_number = day - 304 - leap_year;
                    month_number = 10; 
                } else { 
                    day_number = day - 334 - leap_year;
                    month_number = 11; 
                }

                // first day of month is 1
                util_day_constructor(day, is_leap_year, month_number, day_number as u8 + 1)
             }
        }

        log("Testing for non leap year.");
        testing_year(false);

        log("Testing for leap year.");
        testing_year(true);
    }
}


