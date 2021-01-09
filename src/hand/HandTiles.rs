use crate::HandOption;
use crate::ScoreCalculator;
use crate::tiles::Tile::Tile;

pub struct HandTiles {
    pub hand_tiles: Vec<Tile>,                //手持ちの牌
    pub melded_hand_tiles: Option<Vec<Tile>>, //鳴いた後の牌
    pub winning_tile: Tile,
    pub hand_option: HandOption,
}

impl HandTiles {
    pub fn new(
        hand_tiles: Vec<Tile>,
        melded_hand_tiles: Option<Vec<Tile>>,
        winning_tile: Tile,
        hand_option: HandOption,
    ) -> Self {
        let length = &hand_tiles.len();

        if let Some(melded) = melded_hand_tiles.as_ref() {
            let length = length + melded.len();
        }

        if *length < 13 {
            panic!("The hands are not enough.");
        }
        if *length > 13 {
            panic!("The hands are too many.");
        }

        HandTiles {
            hand_tiles,
            melded_hand_tiles,
            winning_tile,
            hand_option,
        }
    }

    pub fn calc_hand_score(&self) -> u32 {
        let counter = ScoreCalculator::new(self);

        counter.calc_score()
    }
}
