//! Sourced from https://github.com/near-examples/rust-counter
//! For more details check the above link, this example is just for the near-in-minutes video.

use near_sdk::{borsh::{
    self,
    BorshDeserialize,
    BorshSerialize,
}, near_bindgen};


near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract{
    counter: i32,
}

impl Default for Contract{
    fn default() -> Self{
        Contract { counter: 0 }
    }
}

#[near_bindgen]
impl Contract{
    pub fn get(&self) -> i32 {
        self.counter
    }

    pub fn increment(&mut self) -> i32 {
        self.counter += 1;
        self.counter
    }

    pub fn decrement(&mut self) -> i32 {
        self.counter -= 1;
        self.counter
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
            "Assert Error.\n env: {}\naccount: {}\n", 
            env::current_account_id(), 
            &account_id,
        );
    }


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
