use super::SetType::SetType;
use super::WaitingPattern::WaitingPattern;
use crate::tiles::Tile::Tile;

pub struct Set {
    set: Vec<Tile>,
    set_type: SetType,
    waiting_pattern: Option<WaitingPattern>,
}

impl Set {
    fn new(set: Vec<Tile>, set_type: SetType, waiting_pattern: Option<WaitingPattern>) -> Set {
        Set {
            set,
            set_type,
            waiting_pattern,
        }
    }
}
