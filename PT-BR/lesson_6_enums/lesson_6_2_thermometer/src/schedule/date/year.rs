use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


/// Representa um valor de ano. 
/// 
/// O primeiro valor é um inteiro para computação. 
/// O segundo é um String representando o valor formatado.
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Year(i32, String);


impl Year{
    pub fn new(mut value: i32) -> Year {
        let is_negative = value < 0;
        if is_negative{
            value = -value;
        }

        let text: &str = match is_negative{
            false => {
                "BC"
            },
            true => {
                "AD"
            }
        };

        let text: String = format!("{} {}", value, text);
        if is_negative {
            value = -value;
        }

        Year(value, text)
    }

    // /// Numero de anos iniciando em 1970.
    // pub fn from_epoch(value: u64) -> Self {
    //     // No inicio do calculo de data por nanosegundos. Somamos 2 anos ao valor recebido, para garantir que está em sincronia com os leap years.
    //     Self::new(1968 + value as i32)
    // }

    pub fn get(&self) -> i32 {
        // Year é uma tupla, .0 acessa o primeiro valor da tupla.
        // i32 implementa copy, então não precisamos de escrever self.0.clone()
        self.0
    }
}

/// Nos permite usar String::from(&nossoYear)
impl From<&Year> for String{
    fn from(year: &Year) -> String {
        year.1.clone()
    }
}

/// Nos permite usar String::from(nossoYear)
impl From<Year> for String{
    fn from(year: Year) -> String {
        String::from(&year)
    }
}


// Usado para converter o struct para String. Se usarmos instruções como format!, println! ou panic!, esta trait é usada.
impl std::fmt::Display for Year {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(self))
    }
}


