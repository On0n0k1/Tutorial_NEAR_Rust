//! Módulo que representa formato de temperatura.
//! 
//! Pode ser Kelvin, Celsius ou Fahrenheit.
//! 
//! O contrato é inicializado com formato Kelvin.
//! 
//! A instrução #[serde(untagged)] faz com que o enum seja serializado como String em json.
//! 
//!  - Default é implementado. Valor inicial é Celsius::Kelvin;
//!  - PartialEq e Eq implementados. Permitindo comparações entre TempFormats a == b;
//!  - String::from(&formato) converte uma referência &TempFormat para String;
//!  - String::from(formato) converte um TempFormat para String;
//!  - TempFormat::from("a str") para converter um &str para TempFormat;
//!  - TempFormat::from(aString) para converter um String para TempFormat;
//!  - TempFormat::from(&aString) para converter uma referência &String para TempFormat;
//!  - std::fmt::Display implementado. Permitindo o uso desses tipos em macros como println!, format! e panic!;
//! 

use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


/// Representa formato de temperatura (Kelvin, Celsius ou Fahrenheit).
/// 
/// Usado para controle de formato. Podemos ter diversos sensores com diferentes formatos.
/// 
/// Isso garante que todas as possibilidades são aceitas.
/// 
/// Este enum é visto como uma String no formato json.
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum TemperatureUnit {
    Celsius(String),
    Fahrenheit(String),
    Kelvin(String),
}


impl TemperatureUnit {
    /// Constroi uma instância de TempFormat. 
    /// 
    /// Não é case-sensitive. Os valores de String (esquerda) resultam em (direita):
    /// 
    ///  - "celsius", "c" => TempFormat::Celsius("Celsius")
    ///  - "fahrenheit", "f" => TempFormat::Fahrenheit("Fahrenheit")
    ///  - "kelvin", "k" => TempFormat::Kelvin("Kelvin")
    /// 
    /// # Panics
    /// 
    /// Se argumento for inválido.
    /// 
    pub fn new(unit_name: &str) -> Self{
        // This conversion for &str to TemperatureUnit is possible due to From<&str> being implemented
        let lower_case: String = unit_name.to_ascii_lowercase();

        // let's return what matches OR panic!
        match &lower_case[..] {
            "celsius" | "c" => TemperatureUnit::Celsius(String::from("Celsius")),
            "fahrenheit" | "f" => TemperatureUnit::Fahrenheit(String::from("Fahrenheit")),
            "kelvin" | "k" => TemperatureUnit::Kelvin(String::from("Kelvin")),
            invalid_name => panic!("Invalid temperature unit name ({}). Valid args: ['Celsius', 'c', 'Fahrenheit', 'f', 'Kelvin', 'k']", invalid_name),
        }
    }
}


/// O formato padrão de contrato é inicializado como Kelvin. 
/// 
/// Pode ser alterado depois da inicialização de contrato.
/// 
impl Default for TemperatureUnit{
    fn default() -> Self {
        TemperatureUnit::new("k")
    }
}

// Permite comparação parcial entre os tipos TempFormat.
//
// A = B não Garante B = A
//
// A = B e A = C não garante B = C
//
impl PartialEq for TemperatureUnit {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (TemperatureUnit::Celsius(_), TemperatureUnit::Celsius(_)) => true,
            (TemperatureUnit::Fahrenheit(_), TemperatureUnit::Fahrenheit(_)) => true,
            (TemperatureUnit::Kelvin(_), TemperatureUnit::Kelvin(_)) => true,
            (_, _) => false,
        }
    }
}

// Após implementação acima, esta trait permite comparação total entre os tipos TempFormat.
//
// A = B garante B = A
//
// A = B e A = C garante B = C
//
impl Eq for TemperatureUnit {}


/// Conversion to String fro &TemperatureUnit
impl From<&TemperatureUnit> for String{
    fn from(temperature_unit: &TemperatureUnit) -> String {
        match &temperature_unit {
            TemperatureUnit::Celsius(value) => (*value).clone(),
            TemperatureUnit::Kelvin(value) => (*value).clone(),
            TemperatureUnit::Fahrenheit(value) => (*value).clone(),
        }
    }
}

/// Conversion to String from TemperatureUnit
impl From<TemperatureUnit> for String {
    fn from(temperature_unit: TemperatureUnit) -> String {
        String::from(&temperature_unit)
    }
}


/// Conversion to TemperatureUnit from &str
impl From<&str> for TemperatureUnit{
    fn from(temperature_unit_name: &str) -> TemperatureUnit {
        TemperatureUnit::new(temperature_unit_name)
    }
}


/// Conversion to TemperatureUnit from &String
impl From<&String> for TemperatureUnit{
    fn from(temperature_unit_name: &String) -> TemperatureUnit {
        TemperatureUnit::from(&temperature_unit_name[..])
    }
}


/// Conversion to TemperatureUnit from String
impl From<String> for TemperatureUnit{
    fn from(temperature_unit_name: String) -> TemperatureUnit{
        TemperatureUnit::from(&temperature_unit_name[..])
    }
}


/// Usado para converter o enum para String. Se usarmos macros como format!, println! ou panic!, esta trait é usada.
impl std::fmt::Display for TemperatureUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}
