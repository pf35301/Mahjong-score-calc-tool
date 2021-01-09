use crate::TilesType;

pub struct Tile {
    tiles_type: TilesType,
}
//pub useを使え
impl Tile {
    pub fn new(tiles_type: TilesType) -> Self {
        match tiles_type {
            TilesType::Character(number)
            | TilesType::Circle(number)
            | TilesType::Bamboo(number) => {
                if number > 9 {
                    panic!("You need to make the number of tile less than 10");
                }

                Tile { tiles_type }
            }
            _ => Tile { tiles_type },
        }
    }
}
