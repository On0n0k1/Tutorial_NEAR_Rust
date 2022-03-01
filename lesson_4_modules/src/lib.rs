//! Este exemplo foca na administração de módulos.
//! 
//! 
//!
//! A ordem para importação de crates e modulos é, normalmente:
//! 
//!  - padrão (std), 
//!  - outras crates (como near-sdk), 
//!  - crates nesta workspace, 
//!  - e módulos locais.
//!
//! 
//! "mod file" diz ao compilador que o arquivo nesse diretório local deve ser compilado também.
//! Para usarmos o módulo utilizamos "use".
//! 
//! "pub mod" quer dizer que outros módulos podem usar dito módulo, senão é privado.
//! "pub use" pode ser usado para o mesmo propósito.
//! A diferença sendo que arquivos podem ser organizados de certa forma, e módulos de library em outra, mais conveniente.
//! 

// Esta linha está dizendo que existe um arquivo ou diretório com nome "a_module" e deve ser compilado.
// "mod.rs" é uma forma necessária de um diretório ser reconhecido como módulo.
pub mod a_module;
// Esta linha esta dizendo que o arquivo "another_module.rs" é parte deste projeto e deve ser compilado.
mod another_module;

// Esta linha é outra forma de declarar módulos, não precisa de "mod.rs", mas acho mais desorganizado.
mod yet_another_module;

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

// Podemos usar "as" para dar apelidos a funções ou módulos importados.
use a_module::hello as hello;
use a_module::specific_module::hello as hello1;

// pub use torna a função disponível para crates externos.
pub use another_module::hello as hello2;
pub use yet_another_module::hello as hello3;


// A localização atual de "a_deep_function"  é "yet_another_module::internal_module::a_deep_module::a_deep_function."
// Mas este endereço é privado, e "yet_another_modulo" chamou "pub use" para a função. Em outras palavras, mais limpo.
use yet_another_module::a_deep_function as hello4;

// Descomente a linha adiante para receber um aviso de erro "isto é privado".
// use another_module::this_is_private;

pub use a_module::specific_module::this_will_panic;


near_sdk::setup_alloc!();


// Esta função irá fazer nosso código menos bagunçado.
pub fn log(message: &str) {
    env::log(message.as_bytes());
}


#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    stored_phrase: String,
}


#[near_bindgen]
impl Contract{
    /// Retorna a String armazenada.
    pub fn get_phrase(&self) -> String {
        self.stored_phrase.clone()
    }

    /// A função irá imprimir "Hello from crate::a_module" e 
    /// atribuir essa string ao valor armazenado.
    pub fn hello(&mut self) {        
        self.stored_phrase = hello();
        log(&hello());
    }

    /// A função irá imprimir "Hello from 
    /// crate::a_module::specific_module" e atribuir essa string ao 
    /// valor armazenado.
    pub fn hello1(&mut self) {
        self.stored_phrase = hello1();
        log(&hello1());
    }

    /// A função irá imprimir "Hello from another module" e 
    /// atribuir essa string ao valor armazenado.
    pub fn hello2(&mut self) {
        self.stored_phrase = hello2();
        log(&hello2());
    }

    /// A função irá imprimir "Hello from yet_another_module" 
    /// e atribuir essa string ao valor armazenado.
    pub fn hello3(&mut self) {
        self.stored_phrase = hello3();
        log(&hello3());
    }

    /// A função irá imprimir "Called a deep function" e 
    /// atribuir essa string ao valor armazenado.
    pub fn hello4(&mut self) {
        self.stored_phrase = hello4();
        log(&hello4());
    }

    /// Esta função irá entrar em pânico com a mensagem "A panic 
    /// just happened" quando chamado.
    pub fn this_will_panic() {    
        this_will_panic();
    }
}

// Testes estão no diretório tests. Muito útil para projetos grandes.
