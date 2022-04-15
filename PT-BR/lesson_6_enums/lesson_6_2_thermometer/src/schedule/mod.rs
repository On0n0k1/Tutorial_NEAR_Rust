use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    env,
    serde::{ Deserialize, Serialize },
};

pub mod date;
pub mod time;

use date::Date;
use time::Time;

use self::date::month::Month;

use crate::utils::log;


#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Schedule{
    date: Date,
    time: Time,
}


impl Schedule{
    
    fn remainder_from_value(mut value: u64, max: u64) -> (u64, u64) {
        let remainder: u64 = value % max;
        value = (value - remainder) / max;

        (value, remainder)
    }

    fn time_from_nanoseconds(nano: u64) -> (u8, u8, u8, f32) {
        // recolhe todos a porção menor do que um dia.
        let (_, remainder) = Self::remainder_from_value(nano, 24 * 60 * 60 * 1_000_000_000);
        let (hours, remainder) = Self::remainder_from_value(remainder, 60 * 60 * 1_000_000_000);
        let (minutes, remainder) = Self::remainder_from_value(remainder, 60 * 1_000_000_000);
        let (seconds, fraction) = Self::remainder_from_value(remainder, 1_000_000_000);

        (hours as u8, minutes as u8, seconds as u8, fraction as f32 / 1_000_000_000.)
    }

    /// Acho que essa função irá calcular anos leap_year incorretamente. Provavelmente estará um dia errado.
    fn date_from_nanoseconds(nano: u64) -> (i32, String, u8) {
        // Somamos 2 anos ao valor de nanosegundos. Dessa forma estará sincronizado com os leap years, em 1968, em vez de 1970.
        let nano: u64 = nano + 2 * 365 * 24 * 60 * 60 * 1_000_000_000;

        let max: u64 = (365.25 as f64 * 24. * 60. * 60. * 1_000_000_000.) as u64;
        let (year, remainder) = Self::remainder_from_value(nano, max);

        // let mut is_leap_year = false;
        let is_leap_year = year % 4 == 0;

        // if year >= 2 {
        //     // Se for divisivel por 4 e após 1972, true.
        //     is_leap_year = (year % 4) == 0;
        // }

        let max = 24 * 60 * 60 * 1_000_000_000;

        // full_days é uma quantidade de dias entre 366 e 0.
        // O valor remainder (Horas, minutos, segundos, fração) é descartado.
        let (full_days, _) = Self::remainder_from_value(remainder, max);
        let (month, day) = Month::new_from_days(full_days, is_leap_year);
        
        // No inicio do calculo de data por nanosegundos. Somamos 2 anos ao valor recebido, para garantir que está em sincronia com os leap years.
        (year as i32 + 1968, month, day)
    }

    /// Schedule constructor.
    /// 
    /// date: tuple with format (year, month, day).
    /// time: tuple with format (hour, minute, second, fraction of a second).
    /// 
    pub fn new(date: Option<(i32, String, u8)>, time: Option<(u8, u8, u8, f32)>) -> Self {
        let block_time: u64 = env::block_timestamp();
        let (year, month, day) = match date{
            Some(value) => value,
            None => {
                log("Date wasn't specified, using current date.");
                Self::date_from_nanoseconds(block_time)
            },
        };
        let (hour, minute, second, fraction) = match time{
            Some(value) => value,
            None => {
                log("Time wasn't specified, using current time");
                Self::time_from_nanoseconds(block_time)
            }
        };

        log(&format!("Epoch time is {}.", block_time));
        log(&format!("Day: {}, Month: {}, Year: {}", day, &month, year));
        log(&format!("Hour: {}, Minute: {}, Second: {}, Fraction: {}", hour, minute, second, fraction));

        let date: Date = Date::new(day, &month, year);
        let time: Time = Time::new(hour, minute, second, fraction);

        Schedule { 
            date,
            time,
        }
    }
}
