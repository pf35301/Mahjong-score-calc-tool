use super::Honors::Honors;
use super::WindTypes::WindTypes;

pub enum WindAndHonors {
    Wind(WindTypes), //風配
    Honor(Honors), //字牌
}
