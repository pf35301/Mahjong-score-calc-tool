struct Set {
    set: Vec<Tile>,
    pung_set_option: Option<PungOption>,
}

struct PungOption {
    concealed: bool, //暗刻かどうか
}