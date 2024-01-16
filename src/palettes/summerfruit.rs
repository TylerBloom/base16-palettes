use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SummerfruitPalette {
    SummerfruitDark,
    SummerfruitLight,
}
create_palette! {
    SummerfruitDark,
    "151515",
    "202020",
    "303030",
    "505050",
    "B0B0B0",
    "D0D0D0",
    "E0E0E0",
    "FFFFFF",
    "FF0086",
    "FD8900",
    "ABA800",
    "00C918",
    "1FAAAA",
    "3777E6",
    "AD00A1",
    "CC6633",
}
create_palette! {
    SummerfruitLight,
    "FFFFFF",
    "E0E0E0",
    "D0D0D0",
    "B0B0B0",
    "000000",
    "101010",
    "151515",
    "202020",
    "FF0086",
    "FD8900",
    "ABA800",
    "00C918",
    "1FAAAA",
    "3777E6",
    "AD00A1",
    "CC6633",
}
