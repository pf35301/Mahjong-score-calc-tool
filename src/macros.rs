#![allow(unused_macros)]

#[macro_export]
macro_rules! hand_tiles {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push(Tile::new($x));
            )*
            temp_vec
        }
    };
}
