use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShadesOfPurplePalette {
    ShadesOfPurple,
}
create_palette! {
    ShadesOfPurple,
    "1e1e3f",
    "43d426",
    "f1d000",
    "808080",
    "6871ff",
    "c7c7c7",
    "ff77ff",
    "ffffff",
    "d90429",
    "f92a1c",
    "ffe700",
    "3ad900",
    "00c5c7",
    "6943ff",
    "ff2c70",
    "79e8fb",
}
