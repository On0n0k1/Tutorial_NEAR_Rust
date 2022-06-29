use near_sdk::{
    borsh::{
        BorshDeserialize,
        BorshSerialize,
        self,
    },
    serde::{ Deserialize, Serialize },
};

use crate::model::character::Class;


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

impl Stats{
    pub fn new(
        dexterity_base: u32,
        dexterity_rate: u32,
        strength_base: u32,
        strength_rate: u32,
        intelligence_base: u32,
        intelligence_rate: u32,
    ) -> Self {
        let (dexterity, strength, intelligence) = (dexterity_base, strength_base, intelligence_base);

        Stats { 
            dexterity, 
            dexterity_rate,
            dexterity_base,
            strength,
            strength_rate,
            strength_base,
            intelligence,
            intelligence_rate,
            intelligence_base,
        }
    }

    pub fn update(&mut self, level: u32) {
        // dexterity, strength, inteligence
        // all these temporary variables won't exist in the machine code.
        // LLVM compiler optimizes these things away.
        // So don't worry about making your code more readable.

        let dexterity: u32 = self.dexterity_base + self.dexterity_rate * level;
        let strength: u32 = self.strength_base + self.strength_rate * level;
        let inteligence: u32 = self.intelligence_base + self.intelligence_rate * level;

        self.dexterity = dexterity;
        self.strength = strength;
        self.intelligence = inteligence;
    }
}


impl From<&Class> for Stats {
    fn from(class: &Class) -> Stats {
        class.get_stats()
    }
}

impl From<Class> for Stats {
    fn from(class: Class) -> Stats {
        Stats::from(&class)
    }
}
