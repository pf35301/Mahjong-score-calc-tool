use crate::tiles::WindTypes::WindTypes;

pub struct HandOption {
    dora: u32,                 //ドラ数
    honba_number: u32,         //本場数
    is_tsumo: bool,            //ツモ
    is_last_tile: bool,        //海底 河底
    pub seat_wind: WindTypes,  //自風
    pub round_wind: WindTypes, //親かどうか
}

impl HandOption {
    pub fn new(
        dora: u32,
        honba_number: u32,
        is_tsumo: bool,
        is_last_tile: bool,
        seat_wind: WindTypes,
        round_wind: WindTypes,
    ) -> Self {
        HandOption {
            dora,
            honba_number,
            is_tsumo,
            is_last_tile,
            seat_wind,
            round_wind,
        }
    }

    pub fn is_parent(&self) -> bool {
        match &self.seat_wind {
            WindTypes::North => true,
            _ => false,
        }
    }
}
