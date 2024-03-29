use near_sdk::{
    AccountId,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{
        LookupMap, 
    },
    env,
    log,
    near_bindgen,
};

mod model;

use crate::{
    model::{
        character::{
            Character,
            Class,
            Name as CharacterName,
        },
        Errors,
        Chapter,
        player_view,
        Player,
        score::{
            HighScore,
            Score,
            Ranking,
        },
        StorageKey,
    }
};




#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    players: LookupMap<AccountId, Player>,

    ranking: Ranking,
}



// initialize colletions
impl Default for Contract {
    fn default() -> Self {
        let players: LookupMap<AccountId, Player> = LookupMap::new(StorageKey::Players);
        let ranking: Ranking = Ranking::default();

        Contract{
            players,
            ranking,
        }
    }
}


#[near_bindgen]
impl Contract{

    fn is_owner() -> bool {
        let predecessor_account_id: AccountId =  env::predecessor_account_id();
        let current_account_id: AccountId = env::current_account_id();

        predecessor_account_id == current_account_id
    }

    /// Guarantees that the user is not registered.
    fn assert_user_not_registered(&self) -> Result<(), Errors> {
        let predecessor_account_id = env::predecessor_account_id();

        if self.players.contains_key(&predecessor_account_id) ||  Self::is_owner() {
            // Panic because account already exists.
            return Err(Errors::AccountIsAlreadyRegistered(predecessor_account_id));
        }

        Ok(())
    }

    fn assert_user_registered(&self) -> Result<(), Errors> {
        if Self::is_owner(){
            return Ok(());
        }

        let predecessor_account_id = env::predecessor_account_id();

        if ! self.players.contains_key(&predecessor_account_id) {
            // Panic because account already exists.
            return Err(Errors::AccountIsNotRegistered(predecessor_account_id));
        }

        Ok(())
    }

    /// Update the player state.
    /// 
    /// This is going to be replaced by direct pointer access later.
    /// 
    fn save_player(&mut self, player: &Player) -> Result<(), Errors>{
        self.assert_user_registered()?;

        let predecessor_account_id: AccountId = env::predecessor_account_id();

        self.players.insert(&predecessor_account_id, player).unwrap();

        Ok(())
    }

    /// If user does not exist in the database. Ask for registry.
    fn load_player(&self) -> Result<Player, Errors>{
        let predecessor_account_id = env::predecessor_account_id();

        match self.players.get(&predecessor_account_id){
            None => Err(Errors::UserNotRegistered(predecessor_account_id)),
            Some(player) => Ok(player),
        }
    }

    

    /// A user that is not registered can't access the smart contract.
    /// 
    /// Add the predecessor to the smart contract.
    #[handle_result]
    pub fn register_user(&mut self) -> Result<(), Errors> {
        log!("Register User function called.");
        self.assert_user_not_registered()?;

        // While technically we are calling env::predecessor_account_id twice, LLVM compiler will optimize it away.
        // We can write both low level and high level code efficiently in rust. 
        // We just need to decide when a function represents a low level or high level need.
        let predecessor_account_id = env::predecessor_account_id();
        let player = Player::default();
        assert!(self.players.insert(&predecessor_account_id, &player).is_none(), "Smart contract error: Expected None after asserting user is not registered. Got some.");

        log!("User successfully registered.");
        
        Ok(())
    }

    /// User must be registered before using this.
    /// 
    /// Create a character with given name and class.
    /// 
    /// classes: "Warrior" | "Druid" | "Rogue" | "Priest"
    /// 
    #[handle_result]
    pub fn create_character(&mut self, name: String, class: String) -> Result<(), Errors> {
        log!("Create Character function called.");
        
        let class: Class = Class::new(&class)?;
        let character: Character = Character::new(name, class)?;
        let mut player: Player = self.load_player()?;

        player.assign_character(character)?;

        self.save_player(&player)?;

        log!("Character successfully created.");

        Ok(())
    }

    /// Loads and returns an instance of player.
    #[handle_result]
    pub fn check_status(&self) -> Result<player_view, Errors>{
        log!("Check Player Status function called.");

        Self::load_player(&self)?
            .get_view()
    }

    /// Load a character with the given name and return it.
    #[handle_result]
    pub fn load_character(&self, name: String) -> Result<Character, Errors> {
        let player = self.load_player()?;

        player.load_character(name)
    }

    /// Get current ranking.
    pub fn get_ranking(&self) -> Ranking {
        self.ranking.clone()
    }

    /// Get information about the next match.
    #[handle_result]
    pub fn start_match(&mut self) -> Result<Chapter, Errors> {
        log!("Start Match function called.");

        let mut player = self
            .load_player()?;

        let chapter = player.start_match();

        self.save_player(&player)?;

        Ok(chapter)
    }

    /// Report the match finished.
    /// 
    /// Some validations should be done about it.
    /// 
    /// Things like, you can't return a 10 minutes-long match if 10 minutes haven't gone through.
    /// 
    /// A report should be a replay of the entire match. Including the AI of non-player-characters.
    /// 
    #[handle_result]
    pub fn report_match(
        &mut self, 
        character: CharacterName, 
        score: Score, 
        // validation_report: ValidationReport,
    ) -> Result<bool, Errors>{
        log!("Report Match function called.");

        let mut player: Player = self.load_player()?;

        let high_score: Option<HighScore> = player.report_match(character, score)?;

        self.save_player(&player)?;

        // So, if player didn't achieve a highscore of their own, it won't checked in the rankings. 
        // This is to stop a few players from overwhelming the ranking with their name.
        Ok(self.ranking.check_highscore(&high_score))
    }

    /// Change how many players can be stored in the ranking. The larger the list, the more expensive sorting is.
    #[handle_result]
    pub fn set_max_highscore_players(&mut self, max_size: usize) -> Result<(), Errors> {
        if env::signer_account_id() != env::current_account_id() {
            return Result::Err(Errors::OwnerOnly);
        }

        self.ranking.set_max_highscore_players(max_size)?;

        Ok(())
    }
}