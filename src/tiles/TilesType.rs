use crate::WindAndHonors;

pub enum TilesType {
    Character(u32), //萬子
    Circle(u32),    //筒子
    Bamboo(u32),    //索子
    Other(WindAndHonors),
}
