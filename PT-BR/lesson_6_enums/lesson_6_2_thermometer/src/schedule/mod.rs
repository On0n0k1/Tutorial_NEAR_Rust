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


#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Schedule{
    date: Date,
    time: Time,
}


impl Schedule{
    // pub fn new(year: i32, month: &str, day: u8, hour: u8, minute: u8, second: u8, fraction: f32) -> Self {
    //     let date: Date = Date::new(day, month, year);
    //     let time: Time = Time::new(hour, minute, second, fraction);

    //     Schedule{
    //         date,
    //         time,
    //     }
    // }
    fn remainder_from_value(mut value: u64, max: u64) -> (u64, u64) {
        let remainder: u64 = value % max;
        value = (value - remainder) / max;

        (value, remainder)
    }

    /// Transforma nanosegundos para horas, minutos, segundos e fração de segundo.
    // fn time_from_nanoseconds(nano: u64)  -> (u8, u8, u8, f32) {
    //     let (full_seconds, fraction) = Self::remainder_from_value(nano, 1_000_000_000);
    //     // assert!(fraction < 1_000_000_000, "Somehow fraction became greater than a second: {}", fraction);

    //     let (full_minutes, seconds) = Self::remainder_from_value(full_seconds, 60);
    //     let (full_hours, minutes) = Self::remainder_from_value(full_minutes, 60);
    //     let (_, hours) = Self::remainder_from_value(full_hours, 24);
        
    //     (hours as u8, minutes as u8, seconds as u8, (fraction as f32/1_000_000_000 as f32))
    // }

    fn time_from_nanoseconds(nano: u64) -> (u8, u8, u8, f32) {
        // recolhe todos a porção menor do que um dia.
        let (_, remainder) = Self::remainder_from_value(nano, 24 * 60 * 60 * 1_000_000_000);
        let (hours, remainder) = Self::remainder_from_value(remainder, 60 * 60 * 1_000_000_000);
        let (minutes, remainder) = Self::remainder_from_value(remainder, 60 * 1_000_000_000);
        let (seconds, fraction) = Self::remainder_from_value(remainder, 1_000_000_000);

        (hours as u8, minutes as u8, seconds as u8, fraction as f32 / 1_000_000_000.)
    }

    /// Transforma nanosegundos para Ano, mes e dias.
    fn date_from_nanoseconds(nano: u64) -> (i32, String, u8) {
        // Iremos começar do ano.
        let max: u64 = (365.25 as f64 * 24. * 60. * 60. * 1_000_000_000.) as u64;
        let (year, remainder) = Self::remainder_from_value(nano, max);
        
        let mut leap_years = 0;
        let mut is_leap_year = false;
        
        // Leap years começam de 1972 e contam de 4 em 4 anos
        if year >= 2 {
            // Self::remainder_from_value retorna uma tupla. Acessamos o primeiro valor para saber quantos leap years.
            // leap_years = Self::remainder_from_value(year - 2, 4).0;
            // -2 porque os leap years começam em 2. +1 porque o valor 1972 é incluido aos leap years.
            leap_years = (year - 2) % 4 + 1;
            
            // Se for divisivel por 4 e após 1972, true.
            is_leap_year = ((year - 2) % 4) == 0;
        }
        
        let max = 24 * 60 * 60 * 1_000_000_000;

        // Removendo os dias de leap year;
        let remainder = remainder - leap_years * max;

        // full_days é uma quantidade de dias entre 365 e 0.
        // O valor remainder (Horas, minutos, segundos, fração) é descartado.
        let (full_days, _) = Self::remainder_from_value(remainder, max);

        let (month, day) = Month::new_from_days(full_days, is_leap_year);

        // let year: Year = Year::from_epoch(year);
        // let day: Day = Day::new(day as u8, &month, &year);
        
        (year as i32 + 1970, month, day)
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
