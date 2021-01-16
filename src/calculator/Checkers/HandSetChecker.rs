use std::usize;

use super::Set::Set;
use crate::tiles::TilesType::TilesType;
use crate::HandTiles;

pub struct HandSetChecker {}

impl HandSetChecker {
    pub fn Check(hand_tiles: &HandTiles) -> Vec<Set> {
        //検索時にindexで同一性を確認
        /*
        let used_tile = &mut vec![];
        for (i, val) in hand_tiles.hand_tiles.iter().enumerate() {
            if used_tile.iter().find(i) == None {
                let match_tile_count = 0;

                for (i2, val2) in hand_tiles.hand_tiles.iter().enumerate() {
                    if i != i2 && used_tile.iter().find(&i2) == None {

                    }
                }
            }

            used_tile.push(i);
        }

        */

        vec![]
    }
}
