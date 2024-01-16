use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WoodlandPalette {
    Woodland,
}
create_palette! {
    Woodland,
    "231e18",
    "302b25",
    "48413a",
    "9d8b70",
    "b4a490",
    "cabcb1",
    "d7c8bc",
    "e4d4c8",
    "d35c5c",
    "ca7f32",
    "e0ac16",
    "b7ba53",
    "6eb958",
    "88a4d3",
    "bb90e2",
    "b49368",
}
