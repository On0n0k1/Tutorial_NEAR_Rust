// use lesson_4_modules::Contract;

use near_sdk::{
    AccountId,
    env,
    MockedBlockchain,
    testing_env,
    test_utils::VMContextBuilder,
    json_types::ValidAccountId,
};

pub fn env_setup(){
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
