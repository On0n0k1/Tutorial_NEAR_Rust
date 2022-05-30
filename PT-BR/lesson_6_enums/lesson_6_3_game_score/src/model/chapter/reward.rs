use crate::model::{
    character::{
        EXP,
        Level,    
        Character,
    },
    score::Score,
};

pub struct ChapterReward{
    /// Base exp reward.
    exp: EXP,
    /// More exp the higher the score.
    score_multiplier: f32,
    /// The level character is expected to be.
    expected_level: Level,
    /// Less exp the higher the level. More exp the lower the level.
    level_multiplier: f32,
}


impl ChapterReward{


    /// Returns the proportion of extra EXP based on level difference.
    /// 
    /// Max difference is 5.
    /// 
    /// If level is higher than expected, receive less EXP.
    /// 
    /// If level is lower, receive more EXP.
    /// 
    fn compute_level_multiplier(
        mut multiplier: f32, 
        level: Level, 
        expected_level: Level,
    ) -> f32{
        let mut difference = level as f32 - expected_level as f32;

        let result: f32 = 1.0;

        // As example, if proportion is 0.9, having a lower level will multiply the bonus by 1.1 per level (up to 5)
        // 0.8 would multiply the bonus by 1.2/level (up to 5).
        if difference < 0. {
            multiplier = 2. - multiplier;
            difference *= -1.;
        }

        if difference >= 5. {
            return result * difference * difference * difference * difference * difference
        }
        if difference >= 4. {
            return result * difference * difference * difference * difference
        }
        if difference >= 3. {
            return result * difference * difference * difference
        }
        if difference >= 2. {
            return result * difference * difference
        }

        if difference >= 1. {
            return result * difference
        }

        result
    }

    pub fn new(
        exp: EXP,
        score_multiplier: f32,
        expected_level: Level,
        level_multiplier: f32,
    ) -> Self {

        ChapterReward { 
            exp, 
            score_multiplier, 
            expected_level, 
            level_multiplier,
        }
    }

    pub fn compute_reward(
        &self, 
        character: &Character,
        score: &Score,
    ) -> EXP {
        let exp = self.exp;
        let score_bonus = self.score_multiplier * *score as f32;

        let expected_level = self.expected_level;
        let level_multiplier = self.level_multiplier;

        let computed_multiplier = Self::compute_level_multiplier(
            level_multiplier, 
            character.get_level(), 
            expected_level,
        );

        ((exp as f32 + score_bonus as f32) * computed_multiplier) as EXP
    }
}