//! Módulo para um schedule (cronograma).
//! 
//! Um valor de temperatura não possui muita utilidade sem um tempo associado. Este é o objetivo desse tipo.
//! 
//! Composto por Date e Time. Usado em Entry.
//! 
//! Pode ser fornecido como parâmetro ou gerado automaticamente com o momento do sistema.
//! 


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


/// Representa o momento de recebimento da temperatura.
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

    fn time_from_nanoseconds(nano: u64) -> (u8, u8, f32) {
        // recolhe toda a porção menor do que um dia.
        let (_, remainder) = Self::remainder_from_value(nano, 24 * 60 * 60 * 1_000_000_000);
        let (hours, remainder) = Self::remainder_from_value(remainder, 60 * 60 * 1_000_000_000);
        let (minutes, seconds) = Self::remainder_from_value(remainder, 60 * 1_000_000_000);

        (hours as u8, minutes as u8, seconds as f32 / 1_000_000_000.)
    }

    fn date_from_nanoseconds(nano: u64) -> (i32, String, u8) {
        // Somamos 2 anos ao valor de nanosegundos. Dessa forma estará sincronizado com os leap years, em 1968, em vez de 1970.
        let nano: u64 = nano + 2 * 365 * 24 * 60 * 60 * 1_000_000_000;

        let max: u64 = (365.25 as f64 * 24. * 60. * 60. * 1_000_000_000.) as u64;
        let (year, remainder) = Self::remainder_from_value(nano, max);

        let is_leap_year = year % 4 == 0;

        let max = 24 * 60 * 60 * 1_000_000_000;

        // full_days é uma quantidade de dias entre 366 e 0.
        // O valor remainder (Horas, minutos, segundos) é descartado.
        let (full_days, _) = Self::remainder_from_value(remainder, max);
        let (month, day) = Month::new_from_days(full_days, is_leap_year);
        
        // No inicio do calculo de data por nanosegundos. Somamos 2 anos ao valor recebido, para garantir que está em sincronia com os leap years.
        (year as i32 + 1968, month, day)
    }

    /// Construtor de schedule (data e horario).
    /// 
    /// date: tupla com estrutura (year, month, day).
    /// time: tupla com estrutura (hour, minute, second).
    /// 
    pub fn new(date: Option<(i32, String, u8)>, time: Option<(u8, u8, f32)>) -> Self {
        
        // Retorna o momento exato da chamada dessa instrução.
        // Valor em nanosegundos. Quantos nanosegundos desde: 1, jan, 1970.
        let block_time: u64 = env::block_timestamp();
        let (year, month, day) = match date{
            Some(value) => value,
            None => {
                log("Date wasn't specified, using current date.");
                Self::date_from_nanoseconds(block_time)
            },
        };
        
        let (hour, minute, second) = match time{
            Some(value) => value,
            None => {
                log("Time wasn't specified, using current time");
                Self::time_from_nanoseconds(block_time)
            }
        };

        log(&format!("Epoch time is {}.", block_time));
        log(&format!("Day: {}, Month: {}, Year: {}", day, &month, year));
        log(&format!("Hour: {}, Minute: {}, Second: {}", hour, minute, second));

        let date: Date = Date::new(day, &month, year);
        let time: Time = Time::new(hour, minute, second);

        Schedule { 
            date,
            time,
        }
    }
}
