# Lesson 6 - 3 Game Score

[back](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/)

In this example, we will see how to easily escape errors using the ```#[handle_result]``` macro, and also a way to update data about several users at each update (in this case, highscores).

This lesson is a simple simulation of a possible browser "rogue-like" game. "Rogue-likes" are games where your character goes through a series of randomly generated arenas, with randomly generated rewards. A few examples of popular games like this are "The Binding of Isaac", by Edmund McMillen and Florian Himsl and "Hades", by Supergiant Games.

This contract can't be used _(yet)_ for running a real game. But it has much of the structure that could be used for such games. We will go over the structure of the contract, and I would love to have a feedback from readers about their thoughts about this example.

Each player stores information about their characters. There's information on each of their highscores, plus there's a global (limited) highscore that can be updated whenever a new one is achieved. How to best save gas doing these operations is still an open question, but I left my suggestions on this example.

---

## Topics

 - [Building](#building)
 - [Smart Contract API](#smart-contract-api)
 - [How the contract is intended to be used](#how-the-contract-is-intended-to-be-used)
 - [Error management](#error-management)
 - [What each module does](#what-each-module-does)
   - [Chapter](#chapter)
     - [Chapter Reward](#chapter-reward)
   - [Character](#character)
      - [Class](#class)
      - [Stats](#stats)
   - [Player](#player)
      - [View](#view)
    - [Score](#score)
      - [HighScore](#highscore)
      - [Ranking](#ranking)

---

## Building

[top](#topics)

This crate belongs to the workspace at lesson_6_enums. Cargo commands will affect all the crates of the workspace. To specify only this crate, include the option `-p lesson_6_3_game_score`.

Build with:

`cargo build -p lesson_6_3_game_score --target wasm32-unknown-unknown --release`

Test with:

`cargo test -p lesson_6_3_game_score --nocapture`, where `--nocapture` will show output of each test.

---

## Smart Contract API

[top](#topics)

```rust
/// Update the player state.
/// 
/// This is going to be replaced by direct pointer access later.
/// 
fn save_player(&mut self, player: &Player) -> Result<(), Errors>;

/// If a  user does not exist in the database redirect to registry
fn load_player(&self) -> Result<Player, Errors>;

/// A user that is not registered can't access the smart contract.
/// 
/// Add the predecessor to the smart contract.
#[handle_result]
pub fn register_user(&mut self) -> Result<(), Errors>

/// User must be registered before using this.
/// 
/// Create a character with given name and class.
/// 
/// classes: "Warrior" | "Druid" | "Rogue" | "Priest"
#[handle_result]
pub fn create_character(&mut self, name: String, class: String) -> Result<(), Errors>;

/// Loads and returns an instance of player.
#[handle_result]
pub fn check_status(&self) -> Result<player_view, Errors>;

/// Load a character with the given name and return it.
#[handle_result]
pub fn load_character(&self, name: String) -> Result<Character, Errors>;

/// Get current ranking.
pub fn get_ranking(&self) -> Ranking;

/// Get information about the next match.
#[handle_result]
pub fn start_match(&mut self) -> Result<Chapter, Errors>;

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
) -> Result<bool, Errors>;

/// Change how many players can be stored in the ranking. The larger the list, the more expensive sorting is.
#[handle_result]
pub fn set_max_highscore_players(&mut self, max_size: usize) -> Result<(), Errors>;
```

## How the contract is intended to be used

[top](#topics)

We're considering that the game is running on a users' browser. We can't update the game in real time, like most online games would, because of latency and costs. But, when it comes to rogue-like games, each arena is small, and is intended to be finished in a short time: 1 or 2 minutes for each arena. 

We could require, as in-game mechanics, for the arenas to run for a limited time. As the player completes each of these small chapters, the browsers sends a report, a small replay, to the smart contract. The smart contract validates this report, and only then, updates the player's state.

Since the game isn't developed yet our implementation will not receive a report for now and will always consider the validation successful. I encourage you to try implementing a simple game to check how expensive it can be. 

When it comes to ranking of players we'll store it as a small Vector. This is because computing the ranking will become exponentially more expensive the higher the number of players, so let's limit it to something like 100 or 1000 players, and sort the Vector whenever a new entry is achieved.

The first step a browser has to take is call `register_user` so the user is stored in the state. 

 - `check_status` returns information about the current player.
 - `get_ranking` returns the current ranking between players.
 - `create_character` creates a new character associated with that player.
 - `load_character` returns a character owned by that player, with the given name.
 - `start_match` returns information about the current chapter, then resets the timer.
 - `report_match` validates your replay and if successful, give rewards to your character, update highscores and moves to the next chapter.
 - The owner of the smart contract account can call `set_max_highscore_players` to change the max number of players that can exist in ranking.

## Error management

[top](#topics)

In ```/src/model/errors.rs``` we have this enum.

```rust
#[derive(FunctionError, BorshSerialize)]
pub enum Errors{
    AccountIsAlreadyRegistered(AccountId),
    AccountIsNotRegistered(AccountId),
    CharacterNotFound(String),
    CharacterAlreadyExists(String),
    InvalidChapterValidation,
    ChapterNotStarted,
    InvalidCharacterName(String),
    InvalidClassName(String),
    UserNotRegistered(AccountId),
    ExcessiveMaxRankingPlayers(usize, usize),
    OwnerOnly,
}


impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Errors::AccountIsAlreadyRegistered(user) => write!(f, "Username {} is already registered in the database.", user),
            Errors::AccountIsNotRegistered(user) => write!(f, "Tried to update {}, but account is not registered. This a server error, not a user error. Please report it.", user),
            Errors::CharacterNotFound(name) => write!(f, "Character with name {} not found in current account.", name),
            Errors::CharacterAlreadyExists(name) => write!(f, "A character with name {} already exists in this account.", name),
            Errors::InvalidChapterValidation => write!(f, "Failed to validate chapter report"),
            Errors::ChapterNotStarted => write!(f, "Can't attempt to validate chapter without first starting the match."),
            Errors::InvalidCharacterName(name) => write!(f, "Character name starts with an invalid character ({}).", name),
            Errors::InvalidClassName(name) => write!(f, "Invalid name ({}) for character class.", name),
            Errors::UserNotRegistered(user) => write!(f, "User {} needs to create an account before using this service.", user),
            Errors::ExcessiveMaxRankingPlayers(selected, maximum) => write!(f, "Computing ranking is expensive. Can't be higher than {}. Attempted {}.", maximum, selected),
            Errors::OwnerOnly => write!(f, "Only owner may call this function."),
        }
    }
}
```

Each possible value for this enum represents an error that might happen in our project. The trait `std::fmt::Display` is used for turning a type into a String when we use macros like `println!` and `format!`. 

We also derive a new trait called `FunctionError`, and thanks to this trait we don't need to manually call ```env::panic_str``` or assert statements whenever something in the code breaks; we just return the error and the deserializer will raise the error for us. 

```rust
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
```

In this example for creating characters, keep an eye out on three things:
 - The function returns `Result<(), Errors>`.
 - The `#[handle_result]` macro on top of the contract function.
 - `?` operators.

The `?` operator is useful for both Option and Result enums.
 - If we use it with `Option`, it unwraps the value or panics if it's `None`.
 - If we use it with `Result`, it unwraps the value or returns the error type as `Err`.

Notice that the return type of each of the functions with the `?` operator is a `Result` with the same `Err` type as this function (which is `Errors`).

This is a very clean way of handling errors. Don't you agree? We know where the errors are coming from. And we are not cluttering our code with error checking.

Having one enum to represent all the errors in the entire smart contract could easily get bloated in large projects. But that's easy to fix! Just have errors within errors, an enum that contains another enum. 
Create a method that wraps the smaller error into the larger error.

## What each module does

[top](#topics)

Some of these modules are very simple and can be easily expanded. The intention of this lesson is to offer an example of a game architecture, not the game itself. To offer inspiration on this uncharted web3 territory. The less specific the game mechanics are, the easier it is to incorporate to multiple different games.

### Chapter

[top](#topics)

This module represents a single chapter of the game. Each chapter is a small arena that the player stays for a limited time (e.g. 2 minutes). Rogue-like games include randomly generated elements in each chapter. Like random enemies, random bonuses for the player, random rewards, etc.

The `Chapter` type can be seen below: 

```rust
// Calculates score/rewards for each match (chapter)
/// This is meant to represent a game chapter. Each has it's own rewards. Each has it's own validation method.
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Chapter{
    Chapter1(Option<u64>),
    Chapter2(Option<u64>),
    Chapter3(Option<u64>),
}
```

Each Chapter represents a unique "arena". The value within the tuple is used to count match length. 

When a player calls the function "start_match", the tuple stores when the match started. When the player calls "report_match", the chapter will calculater running time `(time_when_finished - time_when_started)` to make sure that the game wasn't run by a machine. 

Let's see how this works: imagine the player calls `start_match`, then, two seconds later, calls `validate_match` with a report that technically lasted two minutes.

The smart contract can't allow that. So, if the time in the report is greater than the time it took since the match started, the contract will panic. That's the only reason for storing time in a chapter.

If the validation is successful, then reward is awarded to the character.

#### Chapter reward

[top](#topics)

Chapter Reward calculates how much "EXP (experience)" was earned from a chapter. 

```rust
pub struct ChapterReward{
    /// Base exp reward.
    pub exp: EXP,
    /// More exp the higher the score.
    pub score_multiplier: f32,
    /// The level character is expected to be.
    pub expected_level: Level,
    /// Less exp the higher the level. More exp the lower the level.
    pub level_multiplier: f32,
}
```

 - `exp`: How much base EXP is earned from this match. Regardless of player performance, they will always receive at least this amount.
 - `expected_level`: What level the player is expected to be at, before starting this chapter. Lower levels means harder matches, so higher rewards.
 - `level_multiplier`: For each level of difference, this will be multiplied an extra time, up to 5 times. Check the values in the tests to see some examples.
 - `score_multiplier`: The higher the score, the more exp. This is a  multiplier that applies to the score.

Check the implementation of `Chapter::compute_reward` for more information on how the bonuses are implemented.


### Character

[top](#topics)

Contains basic information about a game character. Each player has their own list of characters.

```rust
// Attributes are ordered according to priority here, not alphabetic order

/// Represents a playable character in the game.
#[derive(Clone, BorshDeserialize, BorshSerialize, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Character{
    name: Name,
    class: Class,
    level: Level,
    xp: EXP,
    stats: Stats,
    high_score: Score,
}
```

 - `name`: character's name. Will be show on high scores.
 - `class`: class is a model for how a character is built and what it can do later.
 - `level`: character's progression is marked by it's level. Exp raises this value automatically.
 - `xp`: how much exp this character has. Each new level resets this value.
 - `stats`: character actions are determined by their stats. 
 - `high_score`: the highest score achieved by the player in any chapter. If a new highscore is achieved, it will be sent for a comparison with the ranking of players.

#### Class

[top](#topics)

A few examples of classes just to show how each could affect stats in a unique way.

```rust
/// classes: "Warrior" | "Druid" | "Rogue" | "Priest".
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub enum Class{
    Warrior,
    Druid,
    Rogue,
    Priest,
}
```

We implement a constructor:

```rust
impl Class{
    pub fn new(class: &str) -> Result<Class, Errors> {
        let class = match &class.to_ascii_lowercase()[..]{
            "warrior" => { Class::Warrior },
            "druid" => { Class::Druid },
            "rogue" => { Class::Rogue },
            "priest" => { Class::Priest },
            invalid => { return Err(Errors::InvalidClassName(String::from(invalid))) },
        };

        Ok(class)
    }
}
```

Notice how this returns a `Rethe where th; jrror is of type ``Erthe`. Thi; js because we use the `?` operator in the contract methods. There's no need to manually raise errors in the implementation; just return a `Result::Err` instead.

Some type conversions below:

```rust
impl From<&str> for Class{
    fn from(class: &str) -> Class{
        match Class::new(class) {
            Ok(valid) => valid,
            Err(err) => env::panic_str(&format!("{}", err)),
        }
    }
}

impl From<String> for Class{
    fn from(class: String) -> Class {
        Class::from(&class[..])
    }
}

impl From<&String> for Class{
    fn from(class: &String) -> Class{
        Class::from(&class[..])
    }
}

impl From<&Class> for String {
    fn from(class: &Class) -> String {
        let name = match *class{
            Class::Druid => "Druid",
            Class::Priest => "Priest",
            Class::Rogue => "Rogue",
            Class::Warrior => "Warrior",
        };

        String::from(name)
    }
}
```

These are trait implementations for converting one type to another.

`From<&String> for Class` will allow us to pick a string, then attempt to convert it into a class through the function `from`. Here is an example:

```rust
let a = "Druid";
let b: Class = Class::from(a);
```

In the example above we create a `&str` with value "Druid", then create a class using that string slice. 

We implement owned Strings, move on to references of String and finally we end up doing conversion from a reference Class to a String (the reverse way).

#### Stats

[top](#topics)

Stats represent what your character can do in the arena. The success rate of each of their actions. To be used in chapter validation.

Because of the classes shown above. We want to show that each affect the stats in a unique way. So we chose these 3 basic stats: dexterity, strength and inteligence.

```rust
/// The stats of the character that details how character behavior performs.
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Stats{
    dexterity: u32,
    // How much it increases each level
    dexterity_rate: u32,
    // The value at level 1
    dexterity_base: u32,
    strength: u32,    
    strength_rate: u32,
    strength_base: u32,
    intelligence: u32, 
    intelligence_rate: u32,
    intelligence_base: u32,
}
```

Base is the minimum value for that stat. Rate is how much that stat grows with each level. By calling the method `Stats::update` we update the value of each stat whenever the character levels up.


### Player

[top](#topics)

Each user represents one instance of `Player`.

```rust
/// Holds information pertaining to a single user.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Player{
    name: Name,
    high_score: Option<HighScore>,
    // For storing and checking characters by name, doesn't supports iteration
    characters: LookupMap<character::Name, Character>,
    // For storing character names, supports iteration
    character_names: UnorderedSet<character::Name>,
    // With both those above, we can check characters O(1) and iterate through the characters at the same time.
    latest_chapter: Chapter,
}
```

 - `name`: name of the player;
 - `high_score`: high score achieved by the player. `None` if no chapter has been played yet.
 - `characters`: all characters owned by this player;
 - `character_names`: list of character names owned by the player;
 - `latest_chapter`: the next chapter the player is about to play;

The `LookupMap` is constant _O(1)_ for getting and inserting values. The ```UnorderedSet``` is used for iterating through the names. Both are updated simultaneously.

#### View

[top](#topics)

The collections for `characters` and `character_names` can't be serialized to readable json. So we create this type just to use as a return type.

```rust
/// This type exists only needs to be returned when player makes a GET request for their own data.
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct View {
    pub name: player::Name,
    pub high_score: Option<HighScore>,
    pub characters: Vec<Character>,
}
```

We only get player name, highscore and a list of characters for the view, which is all the player needs.

`Vec` is a collection that can be serialized with `serde`. It is updated every time the list of characters changes and its cost increases exponentially the higher the number of characters, so limiting the number of characters is recommended.

### Score

There are two types in this module: HighScore and Ranking.

#### HighScore

[top](#topics)

Represents a character or player's highscore.

```rust
/// Represents a highscore for a player or character.
#[derive(BorshDeserialize, BorshSerialize, Clone, Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct HighScore{
    character: Character,
    score: Score,
    player: AccountId,
}
```

The most important topic to discuss for this type is the following method:

```rust
/// Makes a comparison between the new and old high scores. If a new high_score for the player is achieved
/// update current and return a copy.
pub fn update_highscore(
    current_highscore: &mut Option<HighScore>,
    new_high_score: Option<HighScore>,
) -> Result<Option<HighScore>, Errors> {

    // This match will stop assigning the new highscore if one has not been achieved.
    match (&current_highscore, &new_high_score) {
        (_, None) => { 
            // No highscore was achieved by the character.
            return Ok(None); 
        },
        (None, Some(_)) => {},
        (Some(old_high_score), Some(new_high_score)) => {
            // A character achieved a highscore
            // there is a highscore recorded.
            // makes a comparison and maintain the highest.
            if old_high_score > new_high_score {
                return Ok(None);
            }
        },
    }

    // assign the new highscore
    *current_highscore = new_high_score.clone();

    return Ok(new_high_score);
}
```

Both player and character store an `Option<HighScore>`. So, instead of making functions that pass `HighScore` as arguments we have just one that receives `Option<HighScore>` instead.

When a report is validated, a new highscore for the character may be achieved, so we check all the player's highscores, to confirm if a new highscore for the player was achieved. If this happened, we check if the player highscore is among the ranking of top highscores.

Notice how ranking is only calculated when players achieve their highest score. That saves a lot of computing.

So, `HighScore::update_highscore` receives the current highscore and the latest possible highscore. If latest is higher than the current, we update the current and return a copy.

There is also the possibility of any of these being `None`. Maybe no highscore was achieved. Maybe there were no highscores before this one. The function considers both cases.


#### Ranking

[top](#topics)

Ranking is stored as a vector of `HighScore`. The maximum number of elements stored is limited to reduce sorting costs.

```rust
/// Contains the top ranked matches stored in the smart contract.
/// 
/// It's just a vector. So to avoid high costs sorting.
/// 
/// We limit the max number of entries to RANKSIZE.
/// 
/// Suggestion for change. Store the score of the lowest highscore in the ranking. 
/// Only update and sort the list when a value higher than such is included.
#[derive(BorshDeserialize, BorshSerialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Ranking{
    values: Vec<HighScore>,
    max_size: usize,
    lowest_high_score: Option<HighScore>,
}
```

When a player achieves a new `HighScore`, this function runs: 

```rust
pub fn check_highscore(
    &mut self, 
    high_score: &Option<HighScore>,
) -> bool {
    match high_score {
        None => { 
            // Player didn't achieve a high score.
            false
        },
        Some(high_score) => {
            log!("New High Score for this Player.");

            // Compiler will apply branchless optimization to all these if/else statements.
            if self.lowest_high_score.is_none() {
                // This is the first entry, so just include it.
                self.new_entry(high_score.clone());

                return true;
            } else {
                // This is not the first entry.
                // The list may be full or not.
                let ranking_is_full: bool = self.values.len() == self.max_size;

                if !ranking_is_full {
                    // If the list is not full, just include it.
                    self.new_entry(high_score.to_owned());

                    return true;
                } else {
                    // .unwrap will never panic because of the first "if" above. It is always Some.
                    // We are cloning because unwrap will take ownership of this mutable reference.
                    let lowest_high_score = self.lowest_high_score
                        .clone()
                        .unwrap();

                    if lowest_high_score < *high_score {
                        self.new_entry(high_score.clone());

                        return true;
                    }
                    
                    false
                }
            }
        }
    }
}
```

In summary, what this method does is:
 - If a new HighScore is **not** achieved, do nothing.
 - If the list is empty, just include the entry.
 - If the list is not full, just include the entry.
 - If the list is full, before including the entry, only include if the value is higher than the lowest 
entry in the list.

Each inclusion in the list will result in the list being sorted. Comparison with the lowest entry helps reduce computing costs.


---

Lesson 6 - Game Score :white_check_mark: ... **Done! Congratulations!**
