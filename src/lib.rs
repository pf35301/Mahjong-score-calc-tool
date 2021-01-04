pub mod tiles;

pub use self::tiles::Tile::Tile;
pub use self::tiles::TilesType::TilesType;
pub use self::tiles::WindAndHonors::WindAndHonors;
pub use self::tiles::Honors::Honors;
pub use self::tiles::WindTypes::WindTypes;

pub mod hand;

pub use self::hand::HandOption::HandOption;
pub use self::hand::HandTiles::HandTiles;
pub use self::hand::Yaku::Yaku;

pub(crate) mod calculator;

#[cfg(test)]
mod tests {

    use crate::{HandTiles, HandOption};
    use crate::{Tile, WindTypes, TilesType};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
 
    #[test]
    fn CalcHandTiles() {
        let hand_option = HandOption::new(3, 1, true, WindTypes::North);

        //[TODO] マクロ化
        let hand_tile_vec = vec![
            Tile::new(TilesType::Bamboo(1)), Tile::new(TilesType::Bamboo(1)), Tile::new(TilesType::Bamboo(2)), Tile::new(TilesType::Bamboo(2)),
            Tile::new(TilesType::Bamboo(3)), Tile::new(TilesType::Bamboo(3)), Tile::new(TilesType::Bamboo(7)), Tile::new(TilesType::Bamboo(7)),
            Tile::new(TilesType::Bamboo(8)), Tile::new(TilesType::Bamboo(8)), Tile::new(TilesType::Bamboo(9)), Tile::new(TilesType::Bamboo(9)),
            Tile::new(TilesType::Bamboo(9))
        ];
        let winning_tile = Tile::new(TilesType::Bamboo(9));

        let hand = HandTiles::new(hand_tile_vec, None, winning_tile, hand_option);

        assert_eq!(48000, hand.calc_hand_score());
    }
}
