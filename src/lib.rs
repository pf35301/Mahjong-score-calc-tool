pub mod tiles;
pub mod hand;
pub(crate) mod calculator;

#[cfg(test)]
mod tests {

    use crate::hand::{HandTiles, HandOption};
    use crate::tiles::{Tile, WindTypes, TilesType};

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
 
    #[test]
    fn CalcHandTiles() {
        let hand_option = HandOption::new(3, 1, true, WindTypes::North);
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
