use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


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
    pub fn new(month: &str) -> Self {
        let lower_case: String = month.to_ascii_lowercase();
        
        match &lower_case[..]{
            "january" | "jan" | "janeiro" | "enero" | "ene" => Month::January(String::from("January")),
            "february" | "feb" | "fevereiro" | "fev" | "febrero" => Month::February(String::from("February")),
            "march" | "mar" | "março" | "marzo" => Month::March(String::from("March")),
            "april" | "ap" | "abril" => Month::April(String::from("April")),
            "may" | "maio" | "mayo" => Month::May(String::from("May")),
            "june" | "jun" | "junho" | "junio" => Month::June(String::from("June")),
            "july" | "jul" | "julho" | "julio" => Month::July(String::from("July")),
            "august" | "aug" | "agosto" | "ago" => Month::August(String::from("August")),
            "september" | "sep" | "setembro" | "set" | "septiembre" => Month::September(String::from("September")),
            "october" | "octo" | "outubro" | "out" | "octubre" | "octu" => Month::October(String::from("October")),
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

    /// Recebe um valor entre 0 e 365. Retorna o mês e dia do ano, baseado no dia do ano.
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
            return (String::from("april"), (days - 90 - leap_year) as u8);
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
            return (String::from("octo"), (days - 273 - leap_year) as u8);
        }
        if days <= 334 + leap_year {
            return (String::from("nov"), (days - 304 - leap_year) as u8);
        }

        (String::from("dec"), (days - 334 - leap_year) as u8)  
    }

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

// Usado para converter o struct para String. Se usarmos instruções como format!, println! ou panic!, esta trait é usada.
impl std::fmt::Display for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
