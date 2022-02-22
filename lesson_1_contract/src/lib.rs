//! Fonte <https://github.com/near-examples/rust-counter>
//! 
//! 

// módulos importados
use near_sdk::{
    // Parâmetros recebidos e valores retornados são convertidos para json com esse módulo
    borsh::{
        self,
        BorshDeserialize,
        BorshSerialize,
    },
    // env,
    // Cria o boilerplate necessário para máquinas virtuais NEAR
    near_bindgen,
};

near_sdk::setup_alloc!();

/// Contrato. Este struct contém o estado (state) da máquina virtual.
/// As funções deste struct são as funções do contrato.
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract{
    /// Contador.
    counter: i32
}

// Default é executado quando o contrato não inicializado manualmente.
impl Default for Contract{
    fn default() -> Self{
        Contract { counter: 0 }
    }
}

#[near_bindgen]
impl Contract{

    /// Retorna o contador.
    pub fn get(&self) -> i32 {
        // return self.counter;
        self.counter
    }

    /// Incrementa o contador em 1.
    pub fn increment(&mut self) -> i32 {
        self.counter += 1;
        self.counter
    }

    /// Decrementa o contador em 1.
    pub fn decrement(&mut self) -> i32 {
        self.counter -= 1;
        self.counter
    }
}


// Nossos testes de unidade ficam aqui.
// cfg(test) quer dizer que esse mod só será compilado em ambientes de teste.
#[cfg(test)]
mod tests{
    // super::* importa todos os módulos acima.
    use super::*;
    // alguns módulos que só usaremos em situações de teste
    use near_sdk::{
        // um id de conta como por exemplo "stiltztinkerstein.near"
        AccountId,
        // possui métodos relacionados ao ambiente de execução.
        // por exemplo, se quisermos saber o nome do usuário que executou
        // esse contrato, usaremos uma função no módulo env.
        env,
        // Simula o blockchain
        MockedBlockchain,
        // Macro que inicializa o ambiente de text com o contexto informado.
        testing_env,
        // Usado para criar um contexto de teste.
        test_utils::VMContextBuilder,
        // Simplesmente representa um Id de Conta valido.
        // Um id de conta é um string, mas não é todo string que é um id válido.
        json_types::ValidAccountId,
    };

    /// Essa função não é um teste. É usada pelos testes para simular
    /// um ambiente de teste.
    fn env_setup(){
        // inicializa um construtor de contexto de teste.
        let mut builder: VMContextBuilder = VMContextBuilder::new();

        // atributos que podem ser editados com o builder:
        // current_account_id
        // signer_account_id
        // signer_account_pk
        // precessor_account_id
        // block_index
        // block_timestamp
        // epoch_height
        // account_balance
        // account_locked_balance
        // storage_usage
        // attached_deposit
        // prepaid_gas
        // random_seed
        // is_view

        // string com nome account_id
        let account_id: AccountId = String::from("stiltztinkerstein");

        builder.current_account_id(
            // try_from tenta converter o string acima para um id valido
            // panic se o id é invalido.
            ValidAccountId::try_from(
                account_id.clone()
            ).unwrap()
        );

        // inicializa simulação
        testing_env!(builder.build());

        // Se os dois primeiros parametros não são iguais, 
        // retorna a mensagem de erro seguinte.
        assert_eq!(
            env::current_account_id(),
            account_id, 
            "Erro assert.\n env: {}\naccount: {}\n", 
            env::current_account_id(), 
            &account_id,
        );
    }


    /// Como tem o marcador #[test] vai executar automaticamente
    /// quando realizarmos testes de unidade.
    #[test]
    pub fn get() {
        env_setup();

        let contract: Contract = Contract::default();
        
        assert_eq!(
            contract.get(),
            0
        );
    }

    #[test]
    pub fn increment() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.increment();

        assert_eq!(
            contract.get(),
            1
        );
    }

    #[test]
    pub fn decrement() {
        env_setup();

        let mut contract: Contract = Contract::default();

        contract.decrement();

        assert_eq!(
            contract.get(),
            -1
        );
    }
}
