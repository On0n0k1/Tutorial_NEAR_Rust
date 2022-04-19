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


/// Representa uma data (Dia, Mês, Ano)
/// 
/// Garante que os valores são válidos.
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Date{
    day: Day,
    month: Month,
    year: Year,
}


impl Date{
    /// Constroi uma instância de data.
    /// 
    /// panic se month for um string inválido.
    /// 
    /// panic se day for um valor inválido, referente ao mês/ano.
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
