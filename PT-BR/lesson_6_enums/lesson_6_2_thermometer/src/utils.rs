use near_sdk::serde::{
    Deserialize, Serialize,
};

use crate::entry::Entry;


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
/// Se argumento possuir um index. Retorna um Entry.
/// 
/// Se index for omitido. Retorna um Vec com todas as Entries para aquele usuário.
/// 
/// Não é eficiente quando o contrato possuir muitas entries para cada usuário. 
/// 
/// Mas esperasse que este colete localmente e remova valores antigos para evitar custos desnecessários de computação e armazenamento.
/// 
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
#[serde(untagged)]
pub enum ViewGet{
    Single(Entry),
    Multiple(Vec<Entry>),
}
