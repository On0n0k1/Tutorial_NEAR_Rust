//!
//! - format!
//! - println!
//! - panic!
//! - vec!
//! - setup_alloc!
//! 
//! 
//! 

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};


near_sdk::setup_alloc!();


/// Esta função é usada em ```print_examples```.
/// É compilada em ambientes de teste.
#[cfg(test)]
pub fn log(message: &str) {
    println!("{}", message);
}


/// Esta função é usada em ```print_examples```.
/// É compilada fora de ambientes de teste.
#[cfg(not(test))]
pub fn log(message: &str) {
    env::log(message.as_bytes());
}


#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    // Este contrato não possui estado
}

#[near_bindgen]
impl Contract{

    /// Esta função mostra as diferenças entre println e env::log
    /// Execute com ```cargo test -- --nocapture```, compile, implante e execute em Near.
    /// Note como algumas mensagens aparecem e outras não.
    pub fn print_examples() {
        log("\n\nprint_examples:\n");
        println!("Isto é um println! Não aparece na máquina virtual");
        let a = String::from("algo");
        println!("Isso é outro println com diversos argumentos, {}, {}, {}", 1, "thingy", a);

        env::log(b"Isso e um log. Nao aparece em testes.");
        env::log("Isso é outro log".as_bytes());

        env::log(
            format!("Isso é outro log com diversos argumentos: {}, {}, {}", 1, 2, "3")
            .as_bytes()
        );

        log("Isso é uma mensagem que aparecem em testes e na máquina virtual.");

        log(
            &format!("Isso é outra mensagem com argumentos, {}, {}, {}.",
                5,
                "6",
                format!("7"),
            )
        );
        
        log("\n\n---------------------------------------------------------------------------------\n\n");
    }


    pub fn format_examples() {
        log("\n\nformat_examples:\n");
        let message: String = format!("format retorna um String formatado");
        log(&message);

        let an_arg ="terceiro";
        let message = format!("format pode receber argumentos usando {{}}: {}, {}, {}.", 1, "second", an_arg);
    
        log(&message);

        let (first, second, third) = (1, "segundo", an_arg);
        let message = format!("Podemos especificar argumentos format da seguinte forma: {first}, {second}, {third}.");

        log(&message);

        let message = format!("Podemos especificar a ordem dos argumentos format: {1}, {2}, {0}.", third, first, second);

        log(&message);

        let (first, second, third) = (1, 2, 3);
        let message = format!("Podemos fazer inteiros mostrarem um número arbitrário de digitos: {:02}, {:04}, {:6}.", first, second, third);
        log(&message);

        let message = format!("Escolhendo um número de digitos e ordem: {2:02}, {0:4}, {1:06}.", second, third, first);
        log(&message);

        let (first, second, third) = (0.1, 1.23, -2.45);
        let message = format!("Podemos escolher a precisão de números racionais: {:.2}, {:.4}, {:.6}", first, second, third);
        log(&message);

        let message = format!("Podemos escolher a precisão e o número de digitos: {:2.2}, {:04.4}, {:06.6}", first, second, third);
        log(&message);

        let message = format!("Podemos escolher a precisão, o número de digitos e a ordem dos argumentos: {1:02.2}, {2:4.4}, {0:06.6}", third, first, second);
        log(&message);

        let message = format!("Mesmo que acima: {first:2.2}, {second:04.4}, {third:6.6}");

        log(&message);

        log("\n\n----------------------------------------------------------------------\n\n");
    }

    pub fn panic_example() {
        log("\n\npanic_example:\n\n\n");

        log("Macros de pânico são escritos da mesma forma que println e format.");

        let second = 2;
        panic!("Pânico com alguns argumentos: {} {} {}", 1, second, 3);

    }

    pub fn vec_examples() {
        log("\n\nvec_examples:\n");

        let example = vec![1, 2, 3, 4];

        log(&format!("Podemos imprimir vetores com modo debug:\n{:?}\n\n", example));
        
        log(&format!("Podemos imprimir vetores em \"formato legivel\":\n{:#?}\n\n", example));

        log(&format!("Podemos fazer o mesmo com tuplas:\n{:#?}\n\n", (1, 2, 3)));

        log(&format!("Podemos criar vetores com valores padrão:\n{:?}\n\n", vec![0;5]));

        log("Mais informações na lição de coleções (\"collections\").");

        log("\n\n-------------------------------------------------------------------------------\n\n");
    }
}

// Execute testes com o comando a seguir:
// cargo test -- --nocapture --test-threads=1
//
//
// --nocapture faz imprimir todo o output, incluindo testes sucedidos.
// Sem o argumento --test-threads, todos os testes serão async. O que significa que o output será uma bagunça.
//
#[cfg(test)]
mod tests{
    use super::*;
    use near_sdk::{
        MockedBlockchain,
        testing_env,
        test_utils::VMContextBuilder,
    };

    fn env_setup(){
        let builder: VMContextBuilder = VMContextBuilder::new();
        testing_env!(builder.build());

        // O contrato é stateless. Declaramos, mas não usamos.
        let _contract: Contract = Contract::default();
    }

    #[test]
    pub fn print_examples() {
        env_setup();
    
        Contract::print_examples();
    }

    #[test]
    pub fn format_examples() {
        env_setup();
    
        Contract::format_examples();
    }

    #[test]
    pub fn vec_examples() {
        env_setup();
    
        Contract::vec_examples();
    }

    #[test]
    #[should_panic(expected = "Pânico com alguns argumentos: 1 2 3")]
    pub fn panic_example() {
        env_setup();

        Contract::panic_example();
    }
}
