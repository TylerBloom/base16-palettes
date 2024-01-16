use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnikittyPalette {
    UnikittyLight,
    UnikittyDark,
}
create_palette! {
    UnikittyLight,
    "ffffff",
    "e1e1e2",
    "c4c3c5",
    "a7a5a8",
    "89878b",
    "6c696e",
    "4f4b51",
    "322d34",
    "d8137f",
    "d65407",
    "dc8a0e",
    "17ad98",
    "149bda",
    "775dff",
    "aa17e6",
    "e013d0",
}
create_palette! {
    UnikittyDark,
    "2e2a31",
    "4a464d",
    "666369",
    "838085",
    "9f9da2",
    "bcbabe",
    "d8d7da",
    "f5f4f7",
    "d8137f",
    "d65407",
    "dc8a0e",
    "17ad98",
    "149bda",
    "796af5",
    "bb60ea",
    "c720ca",
}
