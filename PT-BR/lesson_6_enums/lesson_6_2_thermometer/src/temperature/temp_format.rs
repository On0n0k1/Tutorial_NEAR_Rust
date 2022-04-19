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
pub enum TempFormat{
    Celsius(String),
    Fahrenheit(String),
    Kelvin(String),
}


impl TempFormat{
    /// Constroi uma instância de TempFormat. 
    /// 
    /// Não é case-sensitive. Os valores de String (esquerda) resultam em (direita):
    /// 
    ///  - "celsius", "c" => TempFormat::Celsius("Celsius")
    ///  - "fahrenheit", "f" => TempFormat::Fahrenheit("Fahrenheit")
    ///  - "kelvin", "k" => TempFormat::Kelvin("Kelvin")
    /// 
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


/// O formato padrão de contrato é inicializado como Kelvin. 
/// 
/// Pode ser alterado depois da inicialização de contrato.
/// 
impl Default for TempFormat{
    fn default() -> Self {
        TempFormat::new("k")
    }
}

// Permite comparação parcial entre os tipos TempFormat.
//
// A = B não Garante B = A
//
// A = B e A = C não garante B = C
//
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

// Após implementação acima, esta trait permite comparação total entre os tipos TempFormat.
//
// A = B garante B = A
//
// A = B e A = C garante B = C
//
impl Eq for TempFormat {}


/// Nos permite utilizar String::from(&esteTipo) para converter o tipo para String.
impl From<&TempFormat> for String{
    fn from(temp_format: &TempFormat) -> String {
        match &temp_format{
            &TempFormat::Celsius(value) => (*value).clone(),
            &TempFormat::Kelvin(value) => (*value).clone(),
            &TempFormat::Fahrenheit(value) => (*value).clone(),
        }
    }
}

/// Nos permite utilizar String::from(esteTipo) para converter o tipo para String.
impl From<TempFormat> for String {
    fn from(temp_format: TempFormat) -> String {
        String::from(&temp_format)
    }
}


/// Nos permite utilizar TempFormat::from("estestr") para converter um &str para TempFormat.
impl From<&str> for TempFormat{
    fn from(temp_format: &str) -> TempFormat {
        TempFormat::new(temp_format)
    }
}


/// Nos permite utilizar Tempformat::from(&esteString) para converter uma referência &String para TempFormat.
impl From<&String> for TempFormat{
    fn from(temp_format: &String) -> TempFormat {
        TempFormat::from(&temp_format[..])
    }
}


/// Nos permite utilizar TempFormat::from(esteString) para converter um String para TempFormat.
impl From<String> for TempFormat{
    fn from(temp_format: String) -> TempFormat{
        TempFormat::from(&temp_format[..])
    }
}


/// Usado para converter o enum para String. Se usarmos macros como format!, println! ou panic!, esta trait é usada.
impl std::fmt::Display for TempFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp_format = String::from(self);

        write!(f, "{}", temp_format)
    }
}
