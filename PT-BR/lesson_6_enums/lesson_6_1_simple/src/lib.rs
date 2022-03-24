use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[allow(unused_imports)]
use near_sdk::{env, near_bindgen};

near_sdk::setup_alloc!();

mod model;

use model::{
    Example0,
    Example1,
    Example2User,
    log,
};


/// Imprime a função sendo chamada.
pub fn log_call(name: &str){
    log(&format!("Calling {}", name));
}


#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]
pub struct Contract {
    example_0: Example0,
    example_1: Example1,
    example_2_user: Example2User,
}


#[near_bindgen]
impl Contract{
    /// Podemos usar instruções match para Strings e &str.
    /// 
    /// Esta função é um exemplo. 
    /// 
    /// Retorna 1, 2, 3, 4, 5, se o argumento for o número.
    /// 
    /// Causa panic se for outro valor.
    pub fn string_match_example(&self, arg: String) -> u32 {

        // Trata a referencia &String como &str
        return match &arg as &str {
            "1" => 1,
            "2" => 2,
            "3" => 3,
            "4" => 4,
            "5" => 5,
            value => panic!("Received {}. Must be 1, 2, 3, 4 or 5.", value),
        }
    }

    /// Retorna example_0.
    pub fn get_example_0(&self) -> Example0 {
        log_call("get_example_0");
        self.example_0.clone()
    }

    /// Retorna example_1.
    pub fn get_example_1(&self) -> Example1 {
        log_call("get_example_1");
        self.example_1.clone()
    }

    /// Retorna example_2.
    pub fn get_example_2(&self) -> Example2User {
        log_call("get_example_2");
        self.example_2_user.clone()
    }

    /// Chama Example0::get_number.
    pub fn example_0_get_number(&self) -> u32 {
        log_call("get_example_0_get_number");
        self.example_0.get_number()
    }

    /// Chama Example0::is_third.
    pub fn example_0_is_third(&self) -> bool {
        log_call("get_example_0_is_third");
        self.example_0.is_third()
    }

    /// Chama Example1::get.
    pub fn example_1_get(&self) -> String {
        log_call("get_example_1_get");
        self.example_1.get()
    }

    /// Chama Example1::is_novalue.
    pub fn example_1_is_novalue(&self) -> bool {
        log_call("get_example_1_is_novalue");
        self.example_1.is_no_value()
    }

    /// Chama Example1::get_an_integer.
    pub fn example_1_get_an_integer(&self) -> String {
        log_call("get_example_1_get_an_integer");
        let the_integer : Option<i32> = self.example_1.get_an_integer();

        match the_integer {
            Some(value) => {
                format!("{}", value)
            },
            None => {
                String::from("")
            }
        }
    }

    /// Chama Example1::has_an_odd_number.
    pub fn example_1_has_an_odd_number(&self) -> bool {
        log_call("get_example_1_has_an_odd_number");
        self.example_1.has_an_odd_number()
    }

    /// Chama Example2User::get_name.
    pub fn example_2_get_name(&self) -> String {
        log_call("get_example_2_get_name");
        self.example_2_user.get_name()
    }

    /// Chama Example2User::has_permission.
    pub fn example_2_has_permission(&self, permission: String) -> bool {
        log_call("get_example_2_has_permission");
        self.example_2_user.has_permission(permission)
    }

    /// Chama Example2User::get_actions.
    /// 
    /// 
    /// Quando retornamos um Vec, o serializer tentará usar serde::json.
    /// A instrução #[result_serializer] nos permite selecionar borsh como serializador.
    #[result_serializer(borsh)]
    pub fn example_2_get_actions(&self) -> Vec<String>{
        log_call("get_example_2_get_actions");

        let result = self.example_2_user.get_actions();

        let result: Vec<String> = match result {
            Err(err) => panic!("Error: {}\n", err),
            Ok(value) => value,
        };

        result
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

        // attributes we can set with the builder:
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
            "Assert Error. env: {} account: {}", 
            env::current_account_id(), 
            &account_id,
        );
    }

    #[test]
    fn string_match_example(){
        let (a1, a2, a3, a4, a5) = (
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("5"),
        );

        env_setup();

        let contract = Contract::default();

        // string_match_example(&self, arg: String)
        assert_eq!(contract.string_match_example(a1), 1);
        assert_eq!(contract.string_match_example(a2), 2);
        assert_eq!(contract.string_match_example(a3), 3);
        assert_eq!(contract.string_match_example(a4), 4);
        assert_eq!(contract.string_match_example(a5), 5);
    }

    /// Testa situação de erro com argumento 0.
    #[test]
    #[should_panic(expected="Received 0. Must be 1, 2, 3, 4 or 5.")]
    fn string_match_example_error(){
        env_setup();
        let contract = Contract::default();

        contract.string_match_example(String::from("0"));
    }
}