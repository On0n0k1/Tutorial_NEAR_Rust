//! Módulo com todas as funcionalidades necessárias para a 
//! representação de mês no contrato.
//! 
//! Para uma fácil implementação rust, utilizamos um enum 
//! com todos os possiveis valores de mês.
//! Mas para representação json, a melhor opção é utilizar 
//! um String, ou número.
//! 
//! Configurando serde, podemos utilizar o melhor de ambos 
//! os casos. Primeiro, declaramos o enum da seguinte forma:
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
//! Se o valor de Month for 
//! Month::December(String::from("December")), 
//! por exemplo, o valor de estado aparecerá 
//! como month: {December: "December"}.
//! 
//! Mas, se incluirmos o atributo macro 
//! serde(untagged), da seguinte forma:
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
//! Um valor de mês 
//! Month::december(String::from("December")), 
//! irá aparecer simplesmente como month: "December". 
//! O que é muito mais user-friendly.
//! 
//!  - u8::from(&month) converte um mês para um inteiro 
//! de 0 a 11. Não consome o mês;
//!  - Month::from(a_u8_var) converte um valor u8 
//! informado para Month. Panic se não for um valor 
//! entre 0 a 11.
//!  - String::from(&month) constroi uma String com 
//! o mesmo valor de Month.
//!  - String::from(month) consome o Month, convertendo-o 
//! para um String.
//!  - std::fmt::Display é implementado para Month. 
//! Podendo ser utilizado em macros como println!, 
//! format! e panic!;
//! 


use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


/// Representa um mês.
/// 
/// Devido a instrução "serde(untagged)" o valor deste 
/// enum é representado por um String.
/// 
/// Pode ser convertido de/para um String.
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
    /// Constroi uma instância de Mês:
    /// 
    /// Os possiveis valores de String na esquerda são 
    /// convertidos para os seguintes valores na direita:
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
    /// Se o argumento não for nenhum dos possiveis acima.
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

    /// Recebe um valor entre 0 e 365. Retorna o mês e 
    /// dia do ano, baseado no dia do ano.
    /// 
    /// Se is_leap_year é true. Aceita uma valor entre 
    /// 0 e 366. 29 fev é uma possibilidade de data.
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
            // inclui feb 29 se for leap year
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

    /// Retorna uma representação String deste Mês.
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

/// Nos permite utilizar u8::from(&nossoMonth) para 
/// conver um mês para u8.
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

/// Nos permite utilizar Month::from(nossou8) para 
/// converter um valor u8 para Month.
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

/// Nos permite usar String::from(nossoMonth)
impl From<&Month> for String{
    fn from(month: &Month) -> String {
        month.get()
    }
}


/// Nos permite usar String::from(nossoMonth)
impl From<Month> for String{
    fn from(month: Month) -> String {
        String::from(&month)
    }
}

/// Usado para converter o struct para String. Se 
/// usarmos instruções como format!, println! ou panic!, 
/// esta trait é usada.
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
    /// Testa a função Month::new_from_days para todos 
    /// os dias do ano, leap year e não leap year.
    fn new_from_days(){

        /// usado por testing year. Argumentos:
        ///  - days: valor entre 0 e 365. 0 e 366 se 
        /// is_leap_year = true.
        ///  - is_leap_year: true se for leap year
        ///  - expected_month: mes esperado, valor entre 
        /// 0 e 12.
        ///  - expected_day: dia esperado, valor entre 
        /// 0 e 31, depende do mês e leap year.
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

        /// Usa util_day_constructor para testar todos 
        /// os dias do ano.
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

                // Primeiro dia do mês é 1. 
                util_day_constructor(day, is_leap_year, month_number, day_number as u8 + 1)
             }
        }

        // Testa a construção de day para todos os dias do ano. Não é leap year.
        log("Testing for non leap year.");
        testing_year(false);

        // Testa a construção de day para todos os dias do ano. É leap year.
        log("Testing for leap year.");
        testing_year(true);
    }
}


