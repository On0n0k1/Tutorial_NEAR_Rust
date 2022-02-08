//! Este módulo irá testar a funcionalidade do contrato.

// common será um módulo com funções úteis para todos outros testes.
mod common;

// Diretório tests age como a própria crate.
// So when we import our own modules, 
// Então quando importamos nossos próprios módulos,
// referimos a estes como "lesson_4_modules::" em vez de "crates::"
use lesson_4_modules::Contract;

use common::env_setup;


#[test]
pub fn get_phrase() {
    env_setup();

    let contract: Contract = Contract::default();

    // Usamos derive(Default) para Contract.
    // o padrão derivado para String é uma string vazia.
    assert_eq!(contract.get_phrase(), "");
}


/// Esta função irá atribuir "Hello from crate::a_module".
#[test]
pub fn hello() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.hello();

    assert_eq!(
        contract.get_phrase(),
        "Hello from crate::a_module",
    );
}


/// Esta função irá atribuir "Hello from crate::a_module::specific_module".
#[test]
pub fn hello1() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.hello1();

    assert_eq!(
        contract.get_phrase(),
        "Hello from crate::a_module::specific_module",
    );
}


/// Esta função irá atribuir "Hello from another_module".
#[test]
pub fn hello2() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.hello2();

    assert_eq!(
        contract.get_phrase(),
        "Hello from another_module",
    );
}


/// Esta função irá atribuir "Hello from yet_another_module".
#[test]
pub fn hello3() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.hello3();

    assert_eq!(
        contract.get_phrase(),
        "Hello from yet_another_module",
    );
}


/// Esta função irá atribuir "Called a deep function".
#[test]
pub fn hello4() {
    env_setup();

    let mut contract: Contract = Contract::default();

    contract.hello4();

    assert_eq!(
        contract.get_phrase(),
        "Called a deep function",
    );
}


/// Esta função entrará em panico com a messagem "A panic has just happened" quando chamado.
// Podemos testar situações que causam pânico.
// expected garante que estamos entrando em pânico pelo motivo certo.
#[test]
#[should_panic(expected = "A panic has just happened")]
pub fn this_will_panic() {
    env_setup();

    // Embora não utilizemos o state, é bom inicializar antes de cada teste.
    let _contract: Contract = Contract::default();

    Contract::this_will_panic();
}
