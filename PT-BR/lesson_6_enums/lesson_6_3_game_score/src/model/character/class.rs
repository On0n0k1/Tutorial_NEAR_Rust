use near_sdk::borsh::{
    BorshDeserialize,
    BorshSerialize,
    self,
};


use near_sdk::env;

use crate::model::{
    character::Stats,
    Errors,
};


#[derive(BorshDeserialize, BorshSerialize, Clone)]
pub enum Class{
    Warrior,
    Druid,
    Rogue,
    Priest,
}

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

impl Class {
    /// Return base stats for given class, used by Character;
    pub fn get_stats(&self) -> Stats {

        let (
            dexterity_base,
            strength_base,
            inteligence_base,
            strength_rate,
            dexterity_rate,
            inteligence_rate,
        ) = match self{
            Class::Druid => (
                5,
                7,
                7,
                1,
                2,
                2,
            ),
            Class::Priest => (
                4,
                5,
                7,
                1,
                2,
                1,
            ),
            Class::Rogue => (
                8,
                4,
                4,
                2,
                1,
                1,
            ),
            Class::Warrior => (
                4,
                8,
                4,
                1,
                2,
                1,
            ),
        };

        Stats::new(
            dexterity_base,
            dexterity_rate,
            strength_base,
            strength_rate,
            inteligence_base,
            inteligence_rate,
        )
    }
}
