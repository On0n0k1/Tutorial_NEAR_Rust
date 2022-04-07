use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize)]
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
            "january" | "jan" | "janeiro" => Month::January(String::from("January")),
            "february" | "feb" | "fevereiro" | "fev" => Month::February(String::from("February")),
            "march" | "mar" | "março" => Month::March(String::from("March")),
            "april" | "ap" | "abril" => Month::April(String::from("April")),
            "may" | "maio" => Month::May(String::from("May")),
            "june" | "jun" | "junho" => Month::June(String::from("June")),
            "july" | "jul" | "julho" => Month::July(String::from("July")),
            "august" | "aug" | "agosto" | "ago" => Month::August(String::from("August")),
            "september" | "sep" | "setembro" | "set" => Month::September(String::from("September")),
            "october" | "octo" | "outubro" | "out" => Month::October(String::from("October")),
            "november" | "nov" | "novembro" => Month::November(String::from("November")),
            "december" | "dec" | "dezembro" | "dez" => Month::December(String::from("December")),
            invalid => panic!("Invalid value for month: {}.", invalid),
        }
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



