use crate::{HandTiles, hand};

pub struct HanAndFuCalculator {
    Han: u32,
    Fu: u32,
}

impl HanAndFuCalculator {
    pub fn new(hand_tiles: &HandTiles) -> Self {
        let result = Self::han_and_fu_count(hand_tiles);

        Self { Han: result.0, Fu: result.1 }
    }

    fn han_and_fu_count(hand_tiles: &HandTiles) -> (u32, u32) {
        
        
        (0, 0)
    }

    pub fn calc_score(&self, hand_tiles: &HandTiles) -> u32 {
        self.calc_han(hand_tiles) + self.calc_fu(hand_tiles)
    }
    
    fn calc_han(&self, hand_tiles: &HandTiles) -> u32 {
        match hand_tiles.hand_option.seat_wind {
            crate::WindTypes::East => {
                
            },
            _ => {
                
            }
        }
    }

    fn calc_fu(&self, hand_tiles: &HandTiles) -> u32 {
        0
    }
}