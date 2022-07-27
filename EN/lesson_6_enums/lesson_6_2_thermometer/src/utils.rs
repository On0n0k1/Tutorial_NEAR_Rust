//! Módulo com ferramentas de suporte.
//! 
//!  - log: Imprime a mesma mensagem em ambientes de teste e em produção.
//!  - ViewGet: Um tipo de saida para a função de contrato view_get. Permite retorno de dois tipos diferentes na mesma função.
//! 

use near_sdk::serde::{
    Deserialize, Serialize,
};

use crate::entry::TemperatureReading;


#[allow(unused_imports)]
use near_sdk::env;

/// Imprime com env::log em produção. Imprime com println em testes.
#[cfg(test)]
pub fn log(msg: &str){
    println!("{}", msg);
}

/// Imprime com env::log em produção. Imprime com println em testes.
#[cfg(not(test))]
pub fn log(msg: &str) {
    env::log(msg.as_bytes());
}


/// Usado para saida da função de contrato view_get.
/// 
///  - Se argumento possuir um index. Retorna um Entry.
///  - Se index for omitido. Retorna um Vec com todas as Entries para aquele usuário.
/// 
/// Não é eficiente quando o contrato possuir muitas entries para cada usuário. 
/// 
/// Mas esperasse que o usuário colete localmente e remova valores antigos para evitar custos desnecessários de computação e armazenamento.
/// 
/// A instrução #[serde(untagged)] faz com que o enum não apareça no json de saida.
/// 
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(TemperatureReading),
    Multiple(Vec<TemperatureReading>),
}
