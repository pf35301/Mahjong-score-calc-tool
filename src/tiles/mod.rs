use super::calculator;
pub struct Tile {
    tilesType: TilesType,
}

impl Tile {
    pub fn new(tilesType: TilesType) -> Self {
        match tilesType {
            TilesType::Character(number) |
            TilesType::Circle(number) |
            TilesType::Bamboo(number)
            => {
                if number > 9 {
                    panic!("You need to make the number of tile less than 10");
                }

                Tile { tilesType }
            },
            _ => Tile { tilesType },
        }
    }
}

pub enum TilesType {
    Character(u32), //萬子
    Circle(u32), //筒子
    Bamboo(u32), //索子
}