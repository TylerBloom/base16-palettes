use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PurpleDreamPalette {
    PurpleDream,
}

create_palette! {
    PurpleDream,
    "100510",
    "302030",
    "403040",
    "605060",
    "bbb0bb",
    "ddd0dd",
    "eee0ee",
    "fff0ff",
    "FF1D0D",
    "CCAE14",
    "F000A0",
    "14CC64",
    "0075B0",
    "00A0F0",
    "B000D0",
    "6A2A3C",
}
