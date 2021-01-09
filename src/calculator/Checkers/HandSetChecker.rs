use crate::HandTiles;
use crate::tiles::TilesType::TilesType;
use super::Set::Set;

pub struct HandSetChecker {}

impl HandSetChecker {
    pub fn Check(hand_tiles: &HandTiles) -> Vec<Set> {
        for i in &hand_tiles.hand_tiles {
            match &i.tiles_type {
                TilesType::Bamboo(number)
                | TilesType::Character(number)
                | TilesType::Circle(number) => {

                }, 
                TilesType::Other(wind_and_honors) => {
                    HandTiles
                }
            }
        }
    }
}