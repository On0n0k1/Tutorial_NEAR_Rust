# Lesson 6 - 3 Game Score

[back](https://github.com/On0n0k1/Tutorial_NEAR_Rust/tree/main/EN/lesson_6_enums/)

In this example, we will see how to easily escape errors using the macro ```#[handle_result]```, and a way to update data 
about several users at each update (highscores in this case).

This lesson is a simple simulation of a possible browser "rogue-like" game. "Rogue-likes" are games where your 
character goes through a series of randomly generated arenas, with randomly generated rewards. A few examples 
of popular games like this are "The Binding of Isaac", by Edmund McMillen and Florian Himsl and "Hades", by Supergiant 
Games.

This contract can't be used (yet) for running a real game. But it has much of the structure that could be used for a 
game as such. We will discuss the structure of this contract, and I would love to have a feedback from readers about their 
thoughts about this example.

Each player store each of their characters. Have information on each of their highscores, and there is a global (limited) 
highscore that can be updated whenever a new one is achieved. How to best save gas doing these operations is still an open 
question, but I left my suggestions on this example.

If you're interested only in the error Management part of this example, check it in the [topics](#topics) below.

## Building

This crate belongs to the workspace at lesson_6_enums. Cargo commands will affect all the crates of the workspace. 
To specify only this crate, include the option ```-p lesson_6_3_game_score```.

Build with:

```cargo build -p lesson_6_3_game_score --target wasm32-unknown-unknown --release```

Test with:

```cargo test -p lesson_6_3_game_score --nocapture```

```--nocapture``` will show output of each test.


## Topics

 - [Contract Api](#contract-api)
 - [How the Contract is Intended to be Used](#how-the-contract-is-intended-to-be-used)
 - [Error Management](#error-management)
 - [What Each Module Does](#what-each-module-does)
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
 - [Next Section](#next-section) 



## Contract API

[(back to top)](#lesson-6---3-game-score)

```rust
// /src/lib.rs

/// Update the player state.
/// 
/// This is going to be replaced by direct pointer access later.
/// 
fn save_player(&mut self, player: &Player) -> Result<(), Errors>;

/// If user does not exist in the database. Ask for registry.
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

[(back to top)](#lesson-6---3-game-score)

We're considering that the game is running on a users' 
browser. We can't update the game real time like most online 
games because of latency and costs. But, when it comes to 
rogue-like games, each arena is small, and is intended to be 
finished in a short time. Something like 1 minute or 2 for each 
arena. We could require, as in-game mechanics, for the arenas 
to run for a limited time. As the player completes each of 
these small chapters, the browsers sends a report, a small 
replay, to the smart contract. The smart contract validates this 
report, and only then, updates the player's state.

Since the game isn't developed yet. This implementation will 
not receive a report and will always consider the validation 
successful. Maybe in next chapter we can try implementing a 
simple game to check how expensive it can be. You're free to 
use this code in your own games. If it makes you rich, send me 
a couple of NEAR please xD.

When it comes to the ranking of players. I decided to store it 
as a small Vector. This is because computing the ranking will 
become exponentially more expensive the higher the number of 
players. So we limit it to something like 100 or 1000 players. 
And sort the Vector whenever a new entry is achieved.

The first step a browser has to take is call ```register_user``` 
so the user is included in the state. 

 - Calling ```check_status``` will return information about 
the current player.
 - Calling ```get_ranking``` will return the current ranking 
between players.
 - Calling ```create_character``` will create a new character 
associated with that player.
 - Calling ```load_character``` will return a character owned 
by that player, with the given name.
 - Calling ```start_match``` will return information about the 
current chapter, then reset a timer.
 - Calling ```report_match``` with your chapter report will 
validate your replay, if successful, give rewards to your 
character, update highscores and move to the next chapter.
 - The owner of the smart contract accound can call 
```set_max_highscore_players``` to change the max number of 
players that can exist in ranking.

## Error Management

[(back to top)](#lesson-6---3-game-score)

In ```/src/model/errors.rs``` we have this enum.

```rust
// /src/model/errors.rs

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

Each possible value for this enum represents an error that 
might happen in our project. The trait ```std::fmt::Display``` 
is used for turning a type into a String when we use macros 
like ```println!``` and ```format!```. We also derive a new 
trait called ```FunctionError```.

Thanks to this type. We don't need to manually can 
```env::panic_str``` or assert statements whenever something 
in the code might break. We just return the error and the 
deserializer will raise the error for us. Here is how it is 
used.

```rust
// /src/lib.rs

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

In this example for creating character, notice these 3 things:
 - The function returns ```Result<(), Errors>```;
 - ```#[handle_result]``` macro on top of the contract function;
 - ```?``` operators;

The ```?``` operator is useful for both Option and Result enums.
 - If we use it in an ```Option```, unwrap the value or panic if 
it's ```None```.
 - If we use it in a ```Result```, unwrap the value or return the 
error type as ```Err```.

Notice that the return type of each of the functions with ```?``` 
operator is a ```Result``` with the same ```Err``` type as this 
function (which is ```Errors```).

This is a very clean way of handling errors. Don't you agree? We 
know where the errors are coming from. And we are not cluttering 
our code with error checking.

Having one enum to represent all the errors in the entire smart 
contract could easily get bloated in large projects. But that's 
easy to fix. Just have errors within errors. An enum that contains 
another enum. Create a method that wraps the smaller error into the 
larger error that encompasses all the others.

## What each module does

[(back to top)](#lesson-6---3-game-score)

To help users get to know what is happening in each module. I 
will offer a brief explanation of each of those ahead.

Some of these modules are very simple and can be easily 
expanded. The intention of this lesson is to offer an example 
of a game architecture. Not the game itself. To offer inspiration 
on this uncharted web3 territory. The less specific the game 
mechanics are, the easier it is to incorporate to multiple 
different games.

### Chapter

[(back to top)](#lesson-6---3-game-score)

This module represents a single chapter of the game. Each chapter 
should be a small arena that the player should stay for a 
limited time (e.g. 2 minutes). Rogue-like games include randomly 
generated elements in each chapter. Like random enemies, random 
bonuses for the player, random rewards...

The type Chapter can be seen below: 

```rust
// /src/model/chapter/mod.rs

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

Each possible for Chapter represents a unique "arena". The value 
within the tuple is used to count match length. 

When a player calls the function "start_match", the tuple stores 
when the match started. When the player calls "report_match", 
the chapter will use the 
```(time_when_finished - time_when_started)``` to make sure that 
the game wasn't run by a machine. Here is an example:

Imagine the player calls ```start_match``` then, 2 seconds later, 
call ```validate_match``` with a report that technically lasted 
2 minutes.

The smart contract can't allow that. So, if the time in the 
report is greater than the time it took since the match started, 
the contract will panic. That's the only reason for storing the 
time in chapter.

If the validation is successful. Returns the reward for the 
character.

#### Chapter Reward

[(back to top)](#lesson-6---3-game-score)

Chapter Reward is a calculator for how much "exp (experience)" 
was earned from a chapter. Each game is different, so you will 
probably change rewards to something more interesting than this.

```rust
// /src/model/chapter/reward.rs


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

 - ```exp```: How much base EXP is earned from this match. Regardless
 of player performance, they will always receive at least this amount.
 - ```expected_level```: What level the player is expected to be 
before starting this chapter. Lower levels means harder matches, so
higher rewards.
 - ```level_multiplier```: For each level of difference, this will be 
multiplied an extra time. Up to 5 times. Check the values in the 
tests to see some examples.
 - ```score_multiplier```: The higher the score, more exp. This is a 
 multiplier that will apply to the achieved score.


Check the implementation of ```Chapter::compute_reward``` for more 
information on how the bonuses are implemented.


### Character

[(back to top)](#lesson-6---3-game-score)

Contains basic information about a game character. Each player has 
their own list of characters.

```rust
// /src/model/character/mods.rs

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

 - ```name```: character's name. Will be show on high scores.
 - ```class```: class is a model for how a character is built and 
what it can do later. We don't go deep into this topic here. It's 
just an example.
 - ```level```: character's progression is marked by it's level. Exp 
raises this value automatically.
 - ```xp```: how much exp this character has. Each new level resets 
this value.
 - ```stats```: character actions are determined by their stats. Class
has information on how stats increase based on level.
 - ```high_score```: the highest score achieved by the player in any 
chapter. If a new highscore is achieved, it will be sent for a 
comparison with the ranking of players.

#### Class

[(back to top)](#lesson-6---3-game-score)

A few examples of classes just to show how each could affect stats in 
a unique way.


```rust
// /src/model/character/class.rs

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

Notice how this returns a Result where the error is of type 
```Errors```. This is because we use the ```?``` operator in the 
contract methods. There's no need to manually raise errors in the 
implementation. Just return a ```Result::Err``` instead.

Some type conversions below:

```rust
// /src/model/character/class.rs

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

```From<&String> for Class``` will allow us to pick a string, then 
attempt to convert it into a class through the function ```from```. 
Here is an example:

```rust
let a = "Druid";
let b: Class = Class::from(a);
```

The example above we create an ```&str``` with value "Druid" then 
create a class using that string as reference. We implement for 
owned Strings. Then references of String. Then we implement 
conversion from a reference Class to a String (the reverse way).

#### Stats

[(back to top)](#lesson-6---3-game-score)

Stats represent what your character can do in the arena. The success 
rate of each of their actions. To be used in chapter validation.

Because of the classes shown above. We want to show that each 
affect the stats in a unique way. So we chose these 3 basic 
stats: dexterity, strength and inteligence.

```rust
// /model/character/stats.rs

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

Base is the minimum value for that stat. Rate is how much that stat 
grows with each level. By calling the method ```Stats::update``` we 
update the value of each stat whenever the character levels up.


### Player

[(back to top)](#lesson-6---3-game-score)

Each user represents one instance of ```Player```.

```rust
// /src/model/player/mod.rs

/// Holds information pertaining to a single user.
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Player{
    name: Name,
    high_score: Option<HighScore>,
    // For storing and checking characters by name, can't iterate.
    characters: LookupMap<character::Name, Character>,
    // For storing character names, can iterate.
    character_names: UnorderedSet<character::Name>,
    // With both those above, we can check characters O(1) and iterate through the characters at the same time.

    latest_chapter: Chapter,
}
```

 - ```name```: name of the player;
 - ```high_score```: high score achieved by the player. It is None 
 - if no chapter has been played yet;
 - ```characters```: all characters owned by this player;
 - ```character_names```: list of character names owned by the player;
 - ```latest_chapter```: the next chapter the player is about to play;

The ```LookupMap``` is O(1) for getting and inserting values. The ```UnorderedSet``` is used for iterating through the names. Both are updated simultaneously.

#### View

[(back to top)](#lesson-6---3-game-score)

The collections for ```characters``` and ```character_names``` 
can't be serialized to readable json. So we create this type 
just to use as a return type.

```rust
// /src/model/

/// This type exists only to be returned when player makes a GET request for their own data.
#[derive(Deserialize, Serialize)]
#[serde(crate = "near_sdk::serde")]
pub struct View {
    pub name: player::Name,
    pub high_score: Option<HighScore>,
    pub characters: Vec<Character>,
}
```

We only get player name, highscore and a list of characters for 
the view. Which is all the player will need.

Vec is a collection that can be serialized with serde. It is 
updated every time the list of characters change. It gets 
exponentially more expensive the higher the number of characters. 
So limiting the number of characters is recommended.

### Score

There are two types in this module: HighScore and Ranking.

#### HighScore

[(back to top)](#lesson-6---3-game-score)

Represents a character or player's highscore.

```rust
// /src/model/score/high_score.rs

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
// /src/model/score/high_score.rs

/// Makes a comparison between the new and old high scores. If a new high_score for the player is achieved, update current and return a copy.
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

So, this happens for player and character. Both store an 
```Option<HighScore>```. So, instead of making functions that 
receive ```HighScore``` as arguments. We have one that receives 
```Option<HighScore>``` instead.

When a report is validated. A highscore for the character may be 
achieved. 

If this happens, we make a comparison with all the player highscores 
to see if a highscore for the player was achieved.

If this happens as well, we check if the player highscore is among 
the ranking of top highscores.

Notice how ranking is only calculated when player achieve their 
highest score. That saves a lot of computing.

So, ```HighScore::update_highscore``` receives the current highscore 
and the latest possible highscore. If latest is higher than the 
current, updates the current, then return a copy of the same.

There is the possibility of any of the values being ```None```. 
Maybe no highscore was achieved. Maybe there were no highscores 
before this one. The function considers both cases.


#### Ranking

[(back to top)](#lesson-6---3-game-score)

Ranking is stored a vector of ```HighScore```. The maximum 
number of elements stored is limited to reduce sorting costs.

```rust
// /src/model/score/ranking.rs

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

When a player achieves a new ```HighScore```, the following 
method is run:

```rust
// /src/model/score/ranking.rs

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
 - If a new HighScore is not achieved, do nothing.
 - If the list is empty, just include the entry.
 - If the list is not full, just include the entry.
 - If the list is full, before including the entry, 
only include if the value is higher than the lowest 
entry in the list.


Each inclusion in the list will result in the 
list being sorted. Comparison with the lowest entry 
helps reduce computing costs.

## Next Section

[(back to top)](#lesson-6---3-game-score)

The next lesson will be about traits.

