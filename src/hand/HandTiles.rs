use crate::Tile;
use crate::HandOption;

pub struct HandTiles {
    hand_tiles: Vec<Tile>, //手持ちの牌
    melded_hand_tiles: Option<Vec<Tile>>, //鳴いた後の牌
    winning_tile: Tile,
    hand_option: HandOption,
}

impl HandTiles {
    pub fn new(hand_tiles: Vec<Tile>, melded_hand_tiles: Option<Vec<Tile>>, winning_tile: Tile, hand_option: HandOption) -> Self {
        let length = &hand_tiles.len();

        if let Some(melded) = melded_hand_tiles.as_ref() {
            let length = length + melded.len();
        }

        if *length != 13 {
            panic!("The hands are not enough.");
        }
        
        HandTiles { hand_tiles, melded_hand_tiles, winning_tile, hand_option }
    }

    pub fn calc_hand_score(&self) -> u32 {
        0
    }
}