//! Módulo de Contrato.
//! 
//! 

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
    utils::{
        log,
        ViewGet,
    },
    entry::Entry,
};


/// Utilizado para acessar o armazenamento do contrato.
/// 
/// Cada nova instância de Vector, LookupMap ou UnorderedSet precisa de um valor único de key.
/// 
/// Usamos este enum como key.
/// 
#[derive(BorshStorageKey, BorshSerialize)]
enum StorageKey {
    Entries,
    Users,
    UserEntry(String),
}


/// API de contrato:
/// 
/// Apenas usuários permitidos (ou owner) podem executar funções call ao contrato.
///
/// Funções:
///  - **allow_user**: inclui um usuário na lista de permissões de input;
///  - **remove_user**: exclui um usuário da lista de permissões de input;
///  - **set_format**: converte o formato de temperatura para outro;
///  - **new_entry**: inclui uma entry de temperatura;
///  - **list_update_entries**: atualiza todas as entries de um usuário ao formato do sistema, retornando os valores;
///  - **clear_entries**: apaga todas as entries associadas à um usuário;
///  - **view_get_format**: função view. Retorna formato armazenado;
///  - **view_get**: função view. Retorna uma entry se index especificado, retorna todos os valores armazenados por um usuário se não especificado;
/// 
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    temp_format: TempFormat,
    entries: LookupMap<AccountId, Vector<Entry>>,
    users: UnorderedSet<AccountId>,
    temp_length: u32,
}


// Na inicialização de contrato,
// inclui dono na lista de usuários permitidos.
impl Default for Contract {
    fn default() -> Self {
        let temp_format = TempFormat::default();

        let mut entries = LookupMap::new(StorageKey::Entries);

        let owner_account: String = env::current_account_id();
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

    // Garante que apenas owner está chamando a função.
    fn assert_owner_only(&self){
        let predecessor: AccountId = env::predecessor_account_id();
        let owner_id: AccountId = AccountId::from(env::current_account_id());

        assert_eq!(predecessor, owner_id, "Only owner's account is allowed to make this function call.");
    }

    // Garante que o chamado é direto. Não pode ser um contrato chamando outro contrato.
    fn assert_no_cross_contract(&self){
        let signer_id: AccountId = env::signer_account_id();
        let predecessor_id: AccountId = env::predecessor_account_id();
        assert_eq!(signer_id, predecessor_id, "Cross-contract calls not allowed.");
    }

    // Garante que apenas usuários permitidos podem chamar funções.
    fn assert_user_allowed(&self) {
        let predecessor_id: AccountId = env::predecessor_account_id();
        let owner_id: AccountId = env::current_account_id();

        // Se a conta dono do contrato está chamando a função.
        if owner_id == predecessor_id {
            return;
        }

        // Se não for a conta dono, e não estiver incluido na lista de permitidos, causa panic.
        assert!(self.users.contains(&predecessor_id), "User not allowed to make this call.");
    }

    /// Inclui usuário na lista de permissões, cria um Vector para armazenamento de entries para este usuário.
    /// 
    /// Apenas owner tem permissão de chamar esta função.
    /// 
    /// # Panics
    ///  - Se for uma chamada cross-contract;
    ///  - Se não for owner;
    ///  - Se **account_id** for um ID de conta inválido;
    ///  - Se usuário ja estiver incluido;
    /// 
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

    /// Remove usuário da lista de permissões.
    /// 
    /// Apenas owner tem permissão de chamar esta função.
    /// 
    /// # Panics
    ///  - Se for cross-contract;
    ///  - Se não for owner;
    ///  - Se o nome de usuário for inválido;
    ///  - Se usuário informado não existir na lista de permissões;
    ///  - Se o nome de usuário informado for o owner;
    /// 
    pub fn remove_user(&mut self, account_id: String){
        self.assert_no_cross_contract();
        self.assert_owner_only();

        // Conta dono é criada na inicialização de contrato. A possibilidade de remover a conta dono seria um problema.
        let owner_id: AccountId = AccountId::from(env::current_account_id());
        assert_ne!(&owner_id[..], &account_id[..], "Owner account can't be removed from contract.");

        log("Called remove_user");

        log("Validating Account ID.");
        let account_id = match ValidAccountId::try_from(account_id){
            Ok(value) => String::from(value),
            Err(err) => panic!("Invalid user account id: {}.", err),
        };

        // Se usuario não estiver contido na lista de permissões, causa panic.
        log("Checking if user exists.");

        let contains: bool = self.users.contains(&account_id);
        assert!(contains, "User {} not found.", &account_id);

        // Remove vetor de entries referente ao usuario.
        let entries: Option<Vector<Entry>> = self.entries.remove(&account_id);
        assert!(entries.is_some(), "Unexpected Behavior. Found user, but didn't find entry list for user.");

        // Ownership do vetor veio do LookupMap para aqui.
        // Limpa o vetor para garantir segurança de memória.
        // Vetor será liberado da memória no fim desta função.
        let mut entries: Vector<Entry> = entries.unwrap();
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

    
    /// Altera formato de temperatura para o valor informado.
    /// 
    /// Não modifica entries armazenadas. Estas são alteradas quando retornadas pela função list_update_entries.
    /// 
    /// Apenas owner tem permissão de chamar esta função.
    /// 
    /// # Panics
    ///  - Se for cross-contract;
    ///  - Se o usuário informado não for encontrado na lista de permissões;
    ///  - Se o usuário não for owner;
    /// 
    pub fn set_format(&mut self, temp_format: String) {
        self.assert_no_cross_contract();
        self.assert_owner_only();

        log("Called set_format");

        let temp_format = TempFormat::new(&temp_format);

        log(
            &format!("Setting default temperature format to {}", &temp_format)
        );

        self.temp_format = temp_format;
    }
    

    // Exemplo de argumento para esta função: '{"time": [11, 32, 10, 0.85], "date": [2022, "feb", 11], "value": 127, "arg_temp": "k" }'

    /// Armazena um valor de temperatura associado à conta de usuário.
    /// 
    /// Date e time são opcionais. Caso não informados, o sistema usará a data e horários do recebimento da mensagem.
    /// 
    /// format é opcional. Se não informado, usará o formato de temperatura do sistema.
    /// 
    /// # Panics
    ///  - Se usuário não tem permissão de acesso;
    ///  - Se hora (time) for um valor negativo ou maior do que 23;
    ///  - Se minuto (time) for um valor negativo ou maior do que 59;
    ///  - Se segundo (time) for um valor negativo ou maior do que 59.99999....
    ///  - Se dia (date) for um valor inválido para o mês e ano;
    ///  - Se mês (date) for um String inválido para mês;
    ///  - Se temp_format for um String inválido;
    /// 
    /// # Exemplos (bash)
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
        let entry: Entry = Entry::new(time, date, &self.temp_format, temp_value, temp_format);

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


    /// Retorna a lista de entries associadas ao usuário, atualiza os valores com os do sistema, caso diferentes.
    /// 
    /// Se account_id for omitido, retorna as entries do usuário que chamou.
    /// 
    /// Apenas owner tem permissão de acessar e atualizar as entries de outros usuários.
    /// 
    /// # Panics
    ///  - Se usuário não tiver permissão de acesso;
    ///  - Se usuário não for owner e estiver tentando atualizar as entries de outros;
    ///  - Se usuário não for encontrado;
    /// 
    pub fn list_update_entries(
        &mut self, 
        account_id: Option<String>,
    ) -> Vec<Entry> {
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
        
        let mut entries: Vector<Entry> = match self.entries.get(&account_id){
            None => panic!("Couldn't find entries for user {}.", account_id),
            Some(value) => value,
        };

        let mut entries_vec = entries.to_vec();

        let temp_format: TempFormat = self.temp_format.clone();
        let mut changed: bool = false;
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

    /// Apaga todas as entries associadas a um usuário.
    /// 
    /// Se account_id for omitido, apaga as entries do usuário que chamou a função.
    /// 
    /// Apenas owner tem permissão de chamar esta função.
    /// 
    /// # Panics
    ///  - Se usuário não for owner;
    ///  - Se id de conta não estiver na lista de permitidos;
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
        
        let entries: Vector<Entry> = match self.entries.remove(&account_id){
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

    /// Retorna formato de temperatura.
    pub fn view_get_format(&self) -> String {
        String::from(&self.temp_format)
    }

    /// Retorna Entry para usuario.
    /// 
    /// Se index não for especificado, retorna todos os valores associados ao usuário.
    /// 
    /// Não converte as temperaturas armazenadas (caso seja diferente do sistema.)
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

