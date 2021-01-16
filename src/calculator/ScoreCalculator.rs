use std::convert::TryFrom;
use std::usize;

use super::Checkers::{HandSetChecker::HandSetChecker, Set::Set};
use crate::hand;
use crate::tiles::Honors::Honors;
use crate::tiles::WindAndHonors::WindAndHonors;
use crate::tiles::WindTypes::WindTypes;
use crate::HandTiles;
use crate::Tile;
use crate::TilesType;

pub struct ScoreCalculator {
    hand_tiles: HandTiles,
    pub HanScore: usize,
    pub FuScore: usize,
}

impl ScoreCalculator {
    pub fn new(hand_tiles: HandTiles) -> Self {
        let result = Self::calc_yaku_and_fu(&hand_tiles);

        Self {
            hand_tiles,
            HanScore: result.0,
            FuScore: result.1,
        }
    }

    //子
    //符 * 4 * 2^(翻数)
    //親
    //符 * 6 * 2^(翻数)
    pub fn calc(self) -> usize {
        match &self.hand_tiles.hand_option.is_parent() {
            false => &self.FuScore * 4 * 2usize.pow(TryFrom::try_from(self.HanScore).unwrap()),
            true => &self.FuScore * 6 * 2usize.pow(TryFrom::try_from(self.HanScore).unwrap()),
        }
    }

    fn calc_yaku_and_fu(hand_tiles: &HandTiles) -> (usize, usize) {
        //国士無双判定

        for val in &hand_tiles.hand_tiles {}

        //七対子判定

        //面子チェック
    }
}

static THIRTEENORPHANS: Vec<Tile> = hand_tiles![
    TilesType::Character(1),
    TilesType::Character(9),
    TilesType::Circle(1),
    TilesType::Circle(9),
    TilesType::Bamboo(1),
    TilesType::Bamboo(9),
    TilesType::Other(WindAndHonors::Wind(WindTypes::North)),
    TilesType::Other(WindAndHonors::Wind(WindTypes::South)),
    TilesType::Other(WindAndHonors::Wind(WindTypes::West)),
    TilesType::Other(WindAndHonors::Wind(WindTypes::East)),
    TilesType::Other(WindAndHonors::Honor(Honors::WhiteDoragon)),
    TilesType::Other(WindAndHonors::Honor(Honors::GreenDoragon)),
    TilesType::Other(WindAndHonors::Honor(Honors::RedDoragon))
];
