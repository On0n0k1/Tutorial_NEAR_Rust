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
    temperature::temp_format::TempFormat,
    utils::log,
    entry::Entry,
};


#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Entries,
    Users,
    UserEntry(String),
}


#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    temp_format: TempFormat,
    entries: LookupMap<AccountId, Vector<Entry>>,
    users: UnorderedSet<AccountId>,
    temp_length: u32,
}


impl Default for Contract {
    fn default() -> Self {
        // let temp_format = format!("{}", TempFormat::default());
        let temp_format = TempFormat::default();

        let mut entries = LookupMap::new(StorageKey::Entries);

        let owner_account: String = String::from(env::current_account_id());
        let owner_vector: Vector<Entry> = Vector::new(StorageKey::UserEntry(String::from(&owner_account)));
        let inserting = entries.insert(&owner_account, &owner_vector);
        assert!(inserting.is_none(), "Something impossible just happened. Created a LookupMap that already had a value stored.");

        let users: UnorderedSet<AccountId> = UnorderedSet::new(StorageKey::Users);

        Contract{
            temp_format,
            entries,
            users,
            temp_length: 0,
        }
    }
}


#[near_bindgen]
impl Contract{
    fn assert_owner_only(&self){
        let signer_id: AccountId = env::signer_account_id();
        let owner_id: AccountId = AccountId::from(env::current_account_id());

        assert_eq!(signer_id, owner_id, "Only owner's account is allowed to make this function call.");
    }

    fn assert_no_cross_contract(&self){
        let signer_id: AccountId = env::signer_account_id();
        let predecessor_id: AccountId = env::predecessor_account_id();
        assert_eq!(signer_id, predecessor_id, "Cross-contract calls not allowed.");
    }

    fn assert_user_allowed(&self) {
        let signer_id: AccountId = env::signer_account_id();
        let owner_id: AccountId = env::current_account_id();

        // Se a conta dono do contrato está chamando a função.
        if owner_id == signer_id {
            return;
        }

        // Se não for a conta dono, e não estiver incluido na lista de permitidos, causa panic.
        assert!(self.users.contains(&signer_id), "User not allowed to make this call.");
    }


    pub fn allow_user(&mut self, account_id: String){
        self.assert_no_cross_contract();
        self.assert_owner_only();

        log("Called allow_user.");

        // Testa se o nome de usuario é válido.
        log("Validating Account ID.");
        let account_id = match ValidAccountId::try_from(account_id){
            Ok(value) => String::from(value),
            Err(err) => panic!("Invalid user account id: {}.", err),
        };

        // Se usuario ja estiver contido na lista de permissões, causa panic.
        log("Checking if user already exists.");
        let contains: bool = self.users.contains(&account_id);
        assert!(!contains, "User {} is already included in allowed list.", &account_id);
        
        // Cria um vetor para entries e inclui ao mapa.
        log("New user detected. Storing User.");
        let user_vector: Vector<Entry> = Vector::new(StorageKey::UserEntry(String::from(&account_id)));
        let inserting = self.entries.insert(&account_id, &user_vector);
        
        // A asserção abaixo deve ser impossivel de falhar. Se esta falhar, o código possui um erro de implementação.
        // Porque é esperado que a asserção acima sempre falhe antes desta.
        assert!(inserting.is_none(), "Unexpected behavior. User is already included in entries.");

        // Insere nome de usuário na lista de usuários permitidos.
        self.users.insert(&account_id);
    }

    
    pub fn set_format(&mut self, temp_format: String) {
        self.assert_no_cross_contract();
        self.assert_owner_only();

        log("Called set_format");
        // let temp_format = String::from(TempFormat::from(temp_format));
        let temp_format = TempFormat::new(&temp_format);

        log(
            &format!("Setting default temperature format to {}", &temp_format)
        );

        self.temp_format = temp_format;
    }
    
    // Exemplo de argumento para esta função: '{"time": [11, 32, 10, 0.85], "date": [2022, "feb", 11], "value": 127, "arg_temp": "k" }'

    /// Armazena um valor de temperatura associado à conta de usuário.
    /// Date e time são opcionais. Caso não informados, o sistema usará a data e horários do recebimento da mensagem.
    /// arg_temp é opcional. Se não informado, usará o formato de temperatura do sistema.
    pub fn new_entry(
        &mut self, 
        time: Option<(u8, u8, u8, f32)>,
        date: Option<(i32, String, u8)>,
        value: f32, 
        arg_temp: Option<String>,
    ){
        self.assert_user_allowed();
        let user: AccountId = env::predecessor_account_id();

        log("Called new_entry.");

        log("Creating Entry.");
        let entry: Entry = Entry::new(time, date, &self.temp_format, value, arg_temp);

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

    pub fn get_format(&self) -> String {
        let temp_format: String = String::from(&self.temp_format);
        
        temp_format
    }

    // pub fn new_entry(&mut self, )
    pub fn list_entries(&self, account_id: Option<String>) -> Vec<Entry> {
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
                    let owner_id: AccountId = AccountId::from(env::current_account_id());

                    assert_eq!(signer_id, owner_id, "Only owner's account is allowed to check entries of others.");
                }

                value
            }
        };

        
        let entries: Vector<Entry> = match self.entries.get(&account_id){
            None => panic!("Couldn't find entries for user {}.", account_id),
            Some(value) => value,
        };

        entries.to_vec()
    }
}

