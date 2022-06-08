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



// Na inicialização de contrato,
// inclui dono na lista de usuários permitidos.
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

    /// Guarantees that the user is not registered.
    fn assert_user_not_registered(&self) -> Result<(), Errors>{
        let predecessor_account_id = env::predecessor_account_id();

        if self.players.contains_key(&predecessor_account_id) {
            // Panic because account already exists.
            return Err(Errors::AccountIsAlreadyRegistered(predecessor_account_id));
        }

        Ok(())
    }

    fn assert_user_registered(&self) -> Result<(), Errors> {
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

    #[handle_result]
    pub fn register_user(&mut self) -> Result<(), Errors> {
        log!("Register User function called.");
        self.assert_user_not_registered()?;

        // While technically we are calling env::predecessor_account_id twice, LLVM compiler will optimize it away.
        // We can write both low level and high level code efficiently in rust. 
        // We just need to know when a function represents a low level or high level need.
        let predecessor_account_id = env::predecessor_account_id();
        let player = Player::default();
        self.players.insert(&predecessor_account_id, &player).unwrap();

        log!("User successfully registered.");
        
        Ok(())
    }


    /// Loads and returns an instance of player.
    #[handle_result]
    pub fn check_status(&self) -> Result<Player, Errors>{
        log!("Check Player Status function called.");

        Self::load_player(&self)
    }

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

        match high_score {
            None => { 
                // Player didn't achieve a high score.
                Ok(false) 
            },
            Some(high_score) => {
                log!("New High Score for this Player.");

                self.ranking.new_entry(&high_score);

                Ok(self.ranking.contains(&high_score))
            }
        }
    }


}