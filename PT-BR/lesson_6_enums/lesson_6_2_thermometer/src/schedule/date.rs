//! Módulo que representa data.
//! 
//! Usado por Schedule.
//! 
//! Composto por Day, Month e Year.
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


/// Representa uma data (Dia, Mês, Ano).
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Date{
    day: Day,
    month: Month,
    year: Year,
}


impl Date{
    /// Constrói uma instância de data.
    /// 
    /// # Panics
    ///  - Se Day for negativo;
    ///  - Se Day maior do que o limite para o mês e ano;
    ///  - Se Month for um String inválido;
    /// 
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
