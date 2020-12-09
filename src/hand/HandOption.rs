use crate::WindTypes;

pub struct HandOption {
    dora: u32,
    honba_number: u32,
    tsumo: bool,
    seat_wind: WindTypes,
}

impl HandOption {
    pub fn new(dora: u32, honba_number: u32, tsumo: bool, seat_wind: WindTypes) -> Self {
        HandOption { dora, honba_number, tsumo, seat_wind }
    }
}