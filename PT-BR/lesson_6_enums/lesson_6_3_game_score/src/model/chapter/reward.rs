use crate::model::{
    character::{
        EXP,
        Level,
    },
    score::Score,
};

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
            return result * multiplier * multiplier * multiplier * multiplier * multiplier
        }
        if difference >= 4. {
            return result * multiplier * multiplier * multiplier * multiplier
        }
        if difference >= 3. {
            return result * multiplier * multiplier * multiplier
        }
        if difference >= 2. {
            return result * multiplier * multiplier
        }

        if difference >= 1. {
            return result * multiplier
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
        character_level: Level,
        score: &Score,
    ) -> EXP {
        let exp = self.exp;
        let score_bonus = self.score_multiplier * *score as f32;

        let expected_level = self.expected_level;
        let level_multiplier = self.level_multiplier;

        let computed_multiplier = Self::compute_level_multiplier(
            level_multiplier, 
            character_level,
            expected_level,
        );

        ((exp as f32 + score_bonus as f32) * computed_multiplier) as EXP
    }
}


#[cfg(test)]
mod tests{
    use super::ChapterReward;


    fn setup_test() -> ChapterReward {
        let (
            exp,
            score_multiplier,
            expected_level,
            level_multiplier,
        ) = (10, 0.8, 10, 0.9);

        ChapterReward::new(exp, score_multiplier, expected_level, level_multiplier)
    }

    #[test]
    fn chapter_reward_new(){
        
        let chapter_reward: ChapterReward = setup_test();

        assert_eq!(chapter_reward.exp, 10);
        assert_eq!(chapter_reward.score_multiplier, 0.8);
        assert_eq!(chapter_reward.expected_level, 10);
        assert_eq!(chapter_reward.level_multiplier, 0.9);
    }

    #[test]
    fn chapter_reward_compute_reward(){
        let chapter_reward: ChapterReward = setup_test();

        // Lower level means more exp, up to 5 levels of difference.
        assert_eq!(chapter_reward.compute_reward(10, &100), 90);
        assert_eq!(chapter_reward.compute_reward(9, &100), 99);
        assert_eq!(chapter_reward.compute_reward(8, &100), 108);
        assert_eq!(chapter_reward.compute_reward(7, &100), 119);
        assert_eq!(chapter_reward.compute_reward(6, &100), 131);
        assert_eq!(chapter_reward.compute_reward(5, &100), 144);
        assert_eq!(chapter_reward.compute_reward(4, &100), 144);
        assert_eq!(chapter_reward.compute_reward(3, &100), 144);
        assert_eq!(chapter_reward.compute_reward(2, &100), 144);

        // Higher level means less exp, up to 5 levels of difference.
        assert_eq!(chapter_reward.compute_reward(10, &100), 90);
        assert_eq!(chapter_reward.compute_reward(11, &100), 81);
        assert_eq!(chapter_reward.compute_reward(12, &100), 72);
        assert_eq!(chapter_reward.compute_reward(13, &100), 65);
        assert_eq!(chapter_reward.compute_reward(14, &100), 59);
        assert_eq!(chapter_reward.compute_reward(15, &100), 53);
        assert_eq!(chapter_reward.compute_reward(16, &100), 53);
        assert_eq!(chapter_reward.compute_reward(17, &100), 53);
        assert_eq!(chapter_reward.compute_reward(18, &100), 53);
    }
}