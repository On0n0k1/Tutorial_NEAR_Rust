//! Smart Contract module

use near_sdk::{
    AccountId,
    BorshStorageKey,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{
        Vector,
        LookupMap, 
        UnorderedSet,
    },
    env,
    json_types::ValidAccountId,
    near_bindgen,
};


near_sdk::setup_alloc!();


use crate::{
    temperature::temp_format::TemperatureUnit,
    utils::{
        log,
        ViewGet,
    },
    entry::TemperatureReading,
};


/// Used to access smart contract blockchain data
/// 
/// Each Vector, LookupMap or UnderorderedSet needs a unique key.
/// So we use this enum as key.
/// 
#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Entries,
    Users,
    UserEntry(String),
}


/// Smrart Contract API
/// 
/// Only owner or allowed user can use call functions
///
/// Functions:
///  - **add_user**: add user to allowed user list.
///  - **remove_user**: remove user from allowed user list.
///  - **set_default_temperature_unit**: converts from one temperature unit to another.
///  - **new_entry**: add a new temperature measurement.
///  - **list_update_entries**: updates all measurements for a user (converting from/to units if necessary).
///  - **clear_entries**: clear all temperature measurements for a user.
///  - **view_get_format**: view function. Returns default temperature unit.
///  - **view_get**: view function. If given an index returns a specific measurement, if not returns all measurements
/// 
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    temp_format: TemperatureUnit,
    entries: LookupMap<AccountId, Vector<TemperatureReading>>,
    users: UnorderedSet<AccountId>,
    temp_length: u32,
}

// initialize smart contract
impl Default for Contract {
    fn default() -> Self {        
        let mut entries = LookupMap::new(StorageKey::Entries);

        let owner_account: String = env::current_account_id();
        let temperature_readings: Vector<TemperatureReading> = Vector::new(StorageKey::UserEntry(String::from(&owner_account)));
        let inserting = entries.insert(&owner_account, &temperature_readings);

        assert!(inserting.is_none(), "Something impossible just happened. Created a LookupMap that already had a value stored.");

        Contract {
            temp_format: TemperatureUnit::default(),
            entries,
            users: UnorderedSet::new(StorageKey::Users),
            temp_length: 0,
        }
    }
}


#[near_bindgen]
impl Contract{
    // assert the owner is the caller
    fn assert_owner_only(&self) {
        let predecessor: AccountId = env::predecessor_account_id();
        let owner_id: AccountId = env::current_account_id();

        assert_eq!(predecessor, owner_id, "Only owner's account is allowed to make this function call.");
    }

    // don't allow cross-contract calls
    fn assert_no_cross_contract(&self) {
        let signer_id: AccountId = env::signer_account_id();
        let predecessor_id: AccountId = env::predecessor_account_id();
        assert_eq!(signer_id, predecessor_id, "Cross-contract calls not allowed.");
    }

    // check user permissions
    fn assert_user_allowed(&self) {
        let predecessor_id: AccountId = env::predecessor_account_id();
        let owner_id: AccountId = env::current_account_id();

        // is the caller the owner? call assert_owner_only
        if owner_id == predecessor_id {
            return;
        }

        // check if user is in the allowed list
        assert!(self.users.contains(&predecessor_id), "User not allowed to make this call.");
    }

    /// Add user to allowed user list.
    /// 
    /// Only owner can call this function.
    /// 
    /// # Panics
    ///  - If cross-contract call.
    ///  - If caller is not owner.
    ///  - If invalid account name.
    ///  - If user already in the allowed user list.
    /// 
    pub fn add_user(&mut self, account_id: String){
        self.assert_no_cross_contract();
        self.assert_owner_only();

        log("Called add_user.");

        // test if account has a well formed format and follows some simple rules... this doesn't mean it ACTUALLY EXISTS in the blockchain!!
        log("Validating Account ID.");
        let account_id = match ValidAccountId::try_from(account_id){
            Ok(value) => String::from(value),
            Err(err) => panic!("Invalid user account id: {}.", err),
        };

        log("Checking if user already exists.");
        let contains: bool = self.users.contains(&account_id);
        assert!(!contains, "User {} is already included in allowed list.", &account_id);
        
        // Create vector for user data
        log("New user detected. Storing User.");
        let user_vector: Vector<TemperatureReading> = Vector::new(StorageKey::UserEntry(String::from(&account_id)));
        let inserting = self.entries.insert(&account_id, &user_vector);
        
        // last check for any implementation error
        assert!(inserting.is_none(), "Unexpected behavior. User is already included in entries.");

        // add user to list
        self.users.insert(&account_id);
    }

    /// Remove usuário da lista de permissões.
    /// 
    /// Apenas owner tem permissão de chamar esta função.
    /// 
    /// # Panics
    ///  - If cross-contract call.
    ///  - If caller is not the owner.
    ///  - If invalid user name.
    /// 
    pub fn remove_user(&mut self, account_id: String){
        self.assert_no_cross_contract();
        self.assert_owner_only();

        // you can't remove the owner
        let owner_id: AccountId = env::current_account_id();
        assert_ne!(&owner_id[..], &account_id[..], "Owner account can't be removed from contract.");

        log("Called remove_user");

        log("Validating Account ID.");
        let account_id = match ValidAccountId::try_from(account_id){
            Ok(value) => String::from(value),
            Err(err) => panic!("Invalid user account id: {}.", err),
        };

        log("Checking if user exists.");
        let contains: bool = self.users.contains(&account_id);
        // panic if user not in list
        assert!(contains, "User {} not found.", &account_id);

        // remove vector for user data
        let entries: Option<Vector<TemperatureReading>> = self.entries.remove(&account_id);
        assert!(entries.is_some(), "Unexpected Behavior. Found user, but didn't find entry list for user.");

        // clear all user data (security)
        let mut entries: Vector<TemperatureReading> = entries.unwrap();
        entries.clear();

        match self.users.remove(&account_id){
            true => {
                log("User successfully removed.");
            },
            false => {
                log("Unexpected Behavior. Account exists in entries but doesn't exist in user list.");
            },
        };
    }

    
    /// Update default temperature unit (system default).
    /// Doesn't modify any existing entries (Data).
    /// Only owner can call this function.
    /// 
    /// # Panics
    ///  - If cross-contract call.
    ///  - If user not in allowed user list.
    ///  - If caller is not owner
    /// 
    pub fn set_default_temperature_unit(&mut self, unit_name: String) {
        self.assert_no_cross_contract();
        self.assert_owner_only();

        log("Called set_default_temperature_unit");

        let temperature_unit = TemperatureUnit::new(&unit_name);

        log(
            &format!("Setting default temperature unit to {}", &temperature_unit)
        );

        self.temp_format = temperature_unit;
    }
    

    /// Stores a new temperature measurement associated with a user.
    /// 
    /// time and date are optional. If not specified, these will be the current date and time. 
    /// format is optional. If not specified, the default temperature unit (system default) will be used.
    /// 
    /// # Panics
    ///  - If user is not on the allowed list
    ///  - If hour is negative or larger than 23.
    ///  - If minute is negative or larger than 59.
    ///  - If second is negative or larger than 59.9
    ///  - If day is invalid for year and month;
    ///  - If month name is an invalid String.
    ///  - If temp_format is an invalid String.
    /// 
    /// # Examples (bash)
    ///  - new_entry '{"temp_value": 100 }'
    ///  - new_entry '{"temp_value": 100, "temp_format": "Celsius"}'
    ///  - new_entry '{"temp_value": 50.5, "temp_format": "Fahrenheit", "date: [2022, "feb", 11]"}'
    ///  - new_entry '{"temp_value": 11.5, "temp_format": "f", "date": [2018, "mar", 27], "time": [10, 50, 9.3453]}'
    ///  - new_entry '{"temp_value": -45.4, "temp_format": "c", "time": [23, 41, 4.443]}'
    ///  - new_entry '{"temp_value": 44.13, "temp_format": "kelvin"}'
    /// 
    pub fn new_entry(
        &mut self, 
        time: Option<(u8, u8, f32)>,
        date: Option<(i32, String, u8)>,
        temp_value: f32, 
        temp_format: Option<String>,
    ){
        self.assert_user_allowed();
        let user: AccountId = env::predecessor_account_id();

        log("Called new_entry.");

        log("Creating Entry.");
        let entry: TemperatureReading = TemperatureReading::new(time, date, &self.temp_format, temp_value, temp_format);

        log("Acquiring entries for this user.");
        let mut entries = match self.entries.get(&user){
            None => panic!("Unexpected Behavior: Failed to find entries for this user."),
            Some(value) => value,
        };
        
        log("Pushing entry to Vector.");
        entries.push(&entry);
        assert!(self.entries.insert(&user, &entries).is_some(), "Failed to replace vector");

        log("Operation Successful.");
    }


    /// Return user data, updating the values to the default temperature unit.
    /// 
    /// If account_id not specified, return data for the caller account.
    /// 
    /// Only owner can change other user's data.
    /// 
    /// # Panics
    ///  - If user is not allowed.
    ///  - If caller is not owner.
    ///  - If user not found.
    /// 
    pub fn list_update_entries(
        &mut self, 
        account_id: Option<String>,
    ) -> Vec<TemperatureReading> {
        self.assert_user_allowed();

        // let account_id: AccountId = env::predecessor_account_id();
        let account_id = match account_id{
            None => {
                env::predecessor_account_id()
            },
            Some(value) => {
                let predecessor = env::predecessor_account_id();

                if predecessor != value {
                    let signer_id: AccountId = env::signer_account_id();
                    let owner_id: AccountId = env::current_account_id();

                    assert_eq!(signer_id, owner_id, "Only owner's account is allowed to check entries of others.");
                }

                value
            }
        };
        
        let mut entries: Vector<TemperatureReading> = match self.entries.get(&account_id){
            None => panic!("Couldn't find entries for user {}.", account_id),
            Some(value) => value,
        };

        let mut entries_vec = entries.to_vec();

        let temp_format: TemperatureUnit = self.temp_format.clone();
        let mut changed: bool = false;
        
        // MW: check index needed?
        let mut index: u64 = 0;

        // entries.to_vec()
        for entry in entries_vec.iter_mut(){
            if entry.update_temp_format(&temp_format) {
                changed = true;
                entries.replace(index, &entry);
            };

            index += 1;
        };

        if changed {
            self.entries.insert(&account_id, &entries);
        }
        
        entries_vec
    }

    /// Clears all user data.
    /// 
    /// If account_id not specified, clear all user data for the caller.
    /// 
    /// Only owner can call this function.
    /// 
    /// # Panics
    ///  - If user is not owner
    ///  - If specified user is not found (no data)
    /// 
    pub fn clear_entries(
        &mut self, 
        account_id: Option<String>,
    ){
        self.assert_owner_only();
        
        let account_id: String = match account_id {
            None => env::predecessor_account_id(),
            Some(value) => {
                log("Validating user account.");

                match ValidAccountId::try_from(value){
                    Ok(account_id) => String::from(account_id),
                    Err(err) => panic!("Invalid user account id: {}.", err),
                }
            }
        };

        assert!(self.users.contains(&account_id), "Account {} not found.", &account_id);
        
        let entries: Vector<TemperatureReading> = match self.entries.remove(&account_id){
            None => panic!("Couldn't find entries for user {}.", account_id),
            Some(mut value) => {
                value.clear();
                value
            },
        };

        assert!(
            self.entries.insert(&account_id, &entries).is_none(),
            "Unexpected behavior, attempted to remove the vector for {}, but it still exists after removing.", 
            &account_id,
        );

        log(&format!("Successfully removed all entries for {}.", &account_id));
    }

    // View Functions

    /// Returns default temperature unit name
    pub fn view_get_format(&self) -> String {
        String::from(&self.temp_format)
    }

    /// Return user data for a given user.
    /// 
    /// If index not specified, return all temperature measurements for a user.
    /// 
    pub fn view_get(
        &self, 
        index: Option<u64>, 
        account_id: String,
    ) -> ViewGet {
        match index{
            None => {
                let result = self.entries
                    .get(&account_id)
                    .unwrap()
                    .to_vec();

                ViewGet::Multiple(result)
            },
            Some(index) => {
                let result = self.entries
                    .get(&account_id)
                    .unwrap()
                    .get(index)
                    .unwrap();

                ViewGet::Single(result)
            }
        }
    }
}

