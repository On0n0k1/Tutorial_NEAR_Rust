use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Serialize, Deserialize },
};

use crate::schedule::date::{
    month::Month,
    year::Year,
};

/// Representa um valor de dia.
/// 
/// Quando serializado, este tipo é visto como um u8.
/// 
/// Garante que é um dia válido para o mês e ano.
/// 
/// Deve ser positivo e menor do que 28-31 dependendo do mês e ano.
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Day(u8);

impl Day{
    /// Constroi uma instância de dia.
    /// 
    /// panic se dia for inválido.
    /// 
    pub fn new(day: u8, current_month: &Month, current_year: &Year) -> Self {
        let day = Day(day);
        day.assert_valid(current_month, current_year);

        day
    }

    /// panic se dia for invalido.
    pub fn assert_valid(&self, current_month: &Month, current_year: &Year) {
        let &Day(day) = self;

        // Coleta o valor do ano.
        let mut current_year: i32 = current_year.get();

        // Se for negativo, converte para positivo
        if current_year < 0 {
            current_year = -current_year;
        }

        // A cada 4 anos, o mês de janeiro possui 29 dias, ao invez de 28.
        // true se for um "leap year".
        let leap_year: bool = (current_year % 4) == 0;
        // converte true para 1, false para 0.
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

        // true se o valor do dia for válido.
        // false se o valor do dia for maior que o valor referente ao mês.
        assert!(day <= max_day,
            "Invalid values for day. Day: {}, Month: {}, Year: {}. Day for given month and year can not be higher than {}.",
                day,
                current_month,
                current_year,
                max_day,
        )
    }
}


/// Nos permite usar u8::from(nossoDay)
impl From<&Day> for u8{
    fn from(day: &Day) -> u8 {
        let &Day(result) = day;

        result
    }
}

/// Nos permite usar u8::from(nossoDay)
impl From<&Day> for String{
    fn from(day: &Day) -> String {
        u8::from(day).to_string()
    }
}


// Usado para converter o struct para String. Se usarmos instruções como format!, println! ou panic!, esta trait é usada.
impl std::fmt::Display for Day {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}


