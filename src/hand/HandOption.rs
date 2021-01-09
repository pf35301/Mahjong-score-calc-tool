use crate::WindTypes;

pub struct HandOption {
    dora: u32,
    honba_number: u32,
    tsumo: bool,
    last_or_first_tile: bool,
    pub seat_wind: WindTypes,
}

impl HandOption {
    pub fn new(
        dora: u32,
        honba_number: u32,
        tsumo: bool,
        last_or_first_tile: bool,
        seat_wind: WindTypes,
    ) -> Self {
        HandOption {
            dora,
            honba_number,
            tsumo,
            last_or_first_tile,
            seat_wind,
        }
    }
}
