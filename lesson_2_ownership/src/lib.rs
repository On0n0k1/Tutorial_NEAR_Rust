use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};


// Dicas sobre documentação:
// comentarios com // não aparecem na documentação.
// comentarios com /// aparecem como descrição para o que estiver a seguir (mod, fn, struct, enum, trait...)
// comments with //! can only be at the start of the file, and represents the description of the entire module.
// comentarios com //! podem apenas existir no inicio do arquivo, representam a descrição de todo o módulo.

// Macro que gera codigo boilerplate para o projeto. Vai ser deprecado nas proximas versões.
near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(Clone, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    name: String,
}

impl Default for Contract{
    fn default() -> Self {
        // Dar um String inicial como exemplo
        return Contract {
            name: String::from("A default string"),
        };
    }
}


#[near_bindgen]
impl Contract{
    // &str é uma referencia para uma string
    // strings entre aspas " " são 'static &str
    // então ter &str como parâmetro permite ter tanto &String como "uma string estatica como essa"
    /// Retorna o tamanho da string.
    fn this_takes_a_reference(name: &str) -> usize { 
        return name.len();
    }

    // Essa função faz o mesmo que o de cima, mas recebe uma String como parâmetro.
    // Teriamos que converter para uma String dessa forma: String::from("essa")
    /// Retorna o tamanho da string.
    fn this_takes_the_ownership(name: String) -> usize {
        // retorna usize, usize é u32 em sistemas 32 bit, u64 em sistemas 64 bit
        name.len()
    }

    /// Retona o tamanho da string armazenada.
    pub fn get_length(&self) -> u32 {
        // Irá chamar ambos os métodos para mostrar que ambos fazem a mesma coisa.
        //
        // Adicionando & antes de cada parametro é o mesmo que dizer:
        // "Estou dando permissão para esta função olhar o valor dessa variável, mas não estou dando permissão para modificá-lo".
        let length_reference: usize = Self::this_takes_a_reference(&self.name);

        // this_takes_the_ownership quer ter possessão de uma String, então precisamos criar uma cópia para essa.
        let length_ownership: usize = Self::this_takes_the_ownership(self.name.clone());

        // Chamando assert_eq para provar que ambas são iguais.
        // Se os valores são diferentes, o código entra em pânico.
        assert_eq!(
            // primeiro parâmetro para comparar
            length_reference, 
            // segundo parâmetro para comparar
            length_ownership, 
            // Se ambas não são iguais, entra em pânico com a mensagem de erro abaixo
            "Ambos tamanhos não são o mesmo {} e {}", length_reference, length_ownership,
        );

        // Converter para u32 porque é um formato simples para json
        // tipos podem ser convertidos usando as traits "into" e "from" também
        length_reference as u32
    }


    /// Retorna o tamanho da String armazenada. Também muda o nome para "Changed name"
    pub fn get_length_again(&mut self) -> u32 {
        // podemos declarar variaveis que armazenam referencias para um outro valor.
        let a_reference: &String = &self.name;
        let _another_reference: &String = &self.name;
        let _yet_another_reference: &String = &self.name;


        // Podemos ter varias referencias imutaveis ao mesmo tempo.
        // Mas não podemos alterar uma variavel enquanto referencias imutáveis existirem.
        // Se precisarmos tirar uma referencia mutavel, não devem haver referencias imutaveis existindo.

        // Descomente a linha adiante para receber um erro devido a referencias existentes.
        // self.name = String::from("Changed name");

        let length = Self::this_takes_a_reference(a_reference);

        // A linha adiante é ok porém, porque as referências acima não são usadas novamente.
        // Como não são usadas novamente, o compilador sabe que pode liberá-las da memória.
        self.name = String::from("Changed name");

        length as u32
    }
}

#[cfg(test)]
mod tests{
    use super::*;
    use near_sdk::{
        AccountId,
        env,
        MockedBlockchain,
        testing_env,
        test_utils::VMContextBuilder,
        json_types::ValidAccountId,
    };

    fn env_setup(){
        let mut builder: VMContextBuilder = VMContextBuilder::new();

        // atributos que podemos modificar com o builder
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

        let account_id: AccountId = String::from("stiltztinkerstein");

        builder.current_account_id(
            ValidAccountId::try_from(
                account_id.clone()
            ).unwrap()
        );

        testing_env!(builder.build());

        assert_eq!(
            env::current_account_id(),
            account_id, 
            "Erro assert. env: {} account: {}", 
            env::current_account_id(), 
            &account_id,
        );
    }

    #[test]
    pub fn get_length() {
        env_setup();
    
        let mut contract: Contract = Contract::default();
    
        // Ambas funções fazem a mesma coisa, então ambas devem retornar o mesmo valor.
        assert_eq!(
            contract.get_length(),
            contract.get_length_again()
        );

        // get_length_again também modifica a string armazenada.
        assert_eq!(
            contract.name,
            "Changed name"
        );
    }
}