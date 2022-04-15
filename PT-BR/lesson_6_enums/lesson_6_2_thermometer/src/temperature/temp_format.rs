use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};

#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum TempFormat{
    Celsius(String),
    Fahrenheit(String),
    Kelvin(String),
}


impl TempFormat{
    /// Construtor. Cria uma instância de TempFormat utilizando uma referência string como parâmetro.
    pub fn new(temp_format: &str) -> Self{
        // Essa conversão de &str para TempFormat é possivel devido a implementação "impl From<&str> for TempFormat{..." abaixo.
        // TempFormat::from(temp_format)

        let lower_case: String = temp_format.to_ascii_lowercase();
        
        let new_format: TempFormat = match &lower_case[..] {
            "celsius" | "c" => TempFormat::Celsius(String::from("Celsius")),
            "fahrenheit" | "f" => TempFormat::Fahrenheit(String::from("Fahrenheit")),
            "kelvin" | "k" => TempFormat::Kelvin(String::from("Kelvin")),
            invalid => panic!("Invalid String for temperature type ({}). Valid args: ['Celsius', 'c', 'Fahrenheit', 'f', 'Kelvin', 'k']", invalid),
        };

        new_format
    }
}


/// O formato padrão de contrato é inicializado como Kelvin. Mas pode ser alterado depois.
impl Default for TempFormat{
    fn default() -> Self {
        TempFormat::new("k")
    }
}


impl PartialEq for TempFormat {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TempFormat::Celsius(_), TempFormat::Celsius(_)) => true,
            (TempFormat::Fahrenheit(_), TempFormat::Fahrenheit(_)) => true,
            (TempFormat::Kelvin(_), TempFormat::Kelvin(_)) => true,
            (_, _) => false,
        }
    }
}

impl Eq for TempFormat {}


impl From<&TempFormat> for String{
    fn from(temp_format: &TempFormat) -> String {
        match &temp_format{
            &TempFormat::Celsius(value) => (*value).clone(),
            &TempFormat::Kelvin(value) => (*value).clone(),
            &TempFormat::Fahrenheit(value) => (*value).clone(),
        }
    }
}

/// Nos permite converter isso para String.
impl From<TempFormat> for String {
    fn from(temp_format: TempFormat) -> String {
        String::from(&temp_format)
    }
}


/// Nos permite tentar converter uma referência &str para TempFormat.
impl From<&str> for TempFormat{
    fn from(temp_format: &str) -> TempFormat {
        TempFormat::new(temp_format)
    }
}


/// Nos permite converter uma referência &String para TempFormat.
impl From<&String> for TempFormat{
    fn from(temp_format: &String) -> TempFormat {
        TempFormat::from(&temp_format[..])
    }
}


/// Nos permite converter uma String para TempFormat.
impl From<String> for TempFormat{
    fn from(temp_format: String) -> TempFormat{
        TempFormat::from(&temp_format[..])
    }
}


/// Usado para converter o enum para String. Se usarmos instruções como format!, println! ou panic!, esta trait é usada.
impl std::fmt::Display for TempFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp_format = String::from(self);

        write!(f, "{}", temp_format)
    }
}








// /// O formato padrão de contrato é inicializado como Kelvin. Mas pode ser alterado depois.
// impl Default for TempFormat{
//     fn default() -> Self {
//         TempFormat::Kelvin
//     }
// }


// /// Nos permite converter &isso para String.
// impl From<&TempFormat> for String{
//     fn from(value: &TempFormat) -> String {
//         match value{
//             &TempFormat::Celsius => String::from("Celsius"),
//             &TempFormat::Fahrenheit => String::from("Fahrenheit"),
//             &TempFormat::Kelvin => String::from("Kelvin"),
//         }
//     }
// }


// /// Nos permite converter isso para String.
// impl From<TempFormat> for String {
//     fn from(value: TempFormat) -> String {
//         String::from(&value)
//     }
// }


// /// Nos permite tentar converter uma referência &str para TempFormat.
// impl From<&str> for TempFormat{
//     fn from(temp_format: &str) -> TempFormat {
//         let lower_case: String = temp_format.to_ascii_lowercase();
        
//         let new_state: TempFormat = match &lower_case[..] {
//             "celsius" | "c" => TempFormat::Celsius,
//             "fahrenheit" | "f" => TempFormat::Fahrenheit,
//             "kelvin" | "k" => TempFormat::Kelvin,
//             invalid => panic!("Invalid String for temperature type ({}). Valid args: ['Celsius', 'c', 'Fahrenheit', 'f', 'Kelvin', 'k']", invalid),
//         };

//         new_state
//     }
// }


// /// Nos permite converter uma referência &String para TempFormat.
// impl From<&String> for TempFormat{
//     fn from(temp_format: &String) -> TempFormat {
//         TempFormat::from(&temp_format[..])
//     }
// }


// /// Nos permite converter uma String para TempFormat.
// impl From<String> for TempFormat{
//     fn from(temp_format: String) -> TempFormat{
//         TempFormat::from(&temp_format[..])
//     }
// }


// /// Usado para converter o enum para String. Se usarmos instruções como format!, println! ou panic!, esta trait é usada.
// impl std::fmt::Display for TempFormat {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let temp_format = String::from(self);

//         write!(f, "{}", temp_format)
//     }
// }
