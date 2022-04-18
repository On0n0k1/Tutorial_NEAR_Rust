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

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Date{
    day: Day,
    month: Month,
    year: Year,
}

impl Date{
    pub fn new(day: u8, month: &str, year: i32) -> Date {
        // Cria uma instância de tipo representando ano.
        let year: Year = Year::new(year);
        // Cria uma instância de tipo representando mês.
        let month: Month = Month::new(month);
        // Cria uma instância de tipo representando dia.
        let day: Day = Day::new(day, &month, &year);

        Date{
            day,
            month,
            year,
        }
    }
}
