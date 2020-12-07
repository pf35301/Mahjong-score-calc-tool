use core::panicking::panic;

use super::tiles::Tile;
use super::tiles::WindTypes;

pub struct HandTiles {
    handTiles: Vec<Tile>, //手持ちの牌
    meldedHandTiles: Option<Vec<Tile>>, //鳴いた後の牌
    winningTile: Tile,
    handOption: HandOption,
}

impl HandTiles {
    fn new(handTiles: Vec<Tile>, meldedHandTiles: Option<Vec<Tile>>, winningTile: Tile, handOption: HandOption) -> Self {
        if (handTiles.len() + meldedHandTiles.unwrap_or(vec![]).len()) != 13 {
            panic!("The hands are not enough.");
        }

        HandTiles { handTiles, meldedHandTiles, winningTile, handOption }
    }
}

struct HandOption {
    dora: u32,
    honbaNumber: u32,
    tsumo: bool,
    seatWind: WindTypes,
}

impl HandOption {
    fn new(dora: u32, honbaNumber: u32, tsumo: bool, seatWind: WindTypes) -> Self {
        HandOption { dora, honbaNumber, tsumo, seatWind }
    }
}