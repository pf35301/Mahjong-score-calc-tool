use crate::HandTiles;

pub struct ScoreCalculator {
    Han: u32,
    Fu: u32,
}

impl ScoreCalculator {
    pub fn new(hand_tiles: &HandTiles) -> Self {
        let result = Self::han_and_fu_count(hand_tiles);

        Self {
            Han: result.0,
            Fu: result.1,
        }
    }

    fn han_and_fu_count(hand_tiles: &HandTiles) -> (u32, u32) {
        (0, 0)
    }

    pub fn calc_score(&self) -> u32 {
        0
    }

    fn calc_han(&self, hand_tiles: &HandTiles) -> u32 {
        0
    }

    fn calc_fu(&self, hand_tiles: &HandTiles) -> u32 {
        0
    }
}
