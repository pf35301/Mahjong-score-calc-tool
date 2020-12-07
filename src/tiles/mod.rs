pub struct Tile {
    tiles_type: TilesType,
}

impl Tile {
    pub fn new(tiles_type: TilesType) -> Self {
        match tiles_type {
            TilesType::Character(number) |
            TilesType::Circle(number) |
            TilesType::Bamboo(number)
            => {
                if number > 9 {
                    panic!("You need to make the number of tile less than 10");
                }

                Tile { tiles_type }
            },
            _ => Tile { tiles_type },
        }
    }
}

pub enum TilesType {
    Character(u32), //萬子
    Circle(u32), //筒子
    Bamboo(u32), //索子
    Other(WindAndHonors),
}

pub enum WindAndHonors {
    Wind(WindTypes),
    Honor(Honors),
}

pub enum WindTypes {
    East,
    South,
    West,
    North,
}

pub enum Honors {
    WhiteDoragon,
    GreenDoragon,
    RedDoragon,
}