//! Módulo com todas as funcionalidades necessárias para o 
//! funcionamento de um ano no contrato.
//! 
//! Ano é representado por uma tupla com um inteiro e um 
//! String.
//! 
//! - String::from(year) converte um Year para String;
//! - String::from(&year) converte uma referência para 
//! String;
//! - std::fmt::Display é implementado. Portanto pode 
//! ser usado em macros como format!, println! e panic!;
//! 


use near_sdk::{
    borsh::{ self, BorshDeserialize, BorshSerialize },
    serde::{ Deserialize, Serialize },
};


/// Tupla representando um valor de ano. 
/// 
/// O primeiro valor (i32) é um inteiro para computação.
///  
/// O segundo é um String representando o valor formatado.
/// 
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Year(i32, String);


impl Year{
    /// Constrói uma instância de Year.
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

    /// Retorna ano como i32.
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


