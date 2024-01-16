use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GigavoltPalette {
    Gigavolt,
}
create_palette! {
    Gigavolt,
    "202126",
    "2d303d",
    "5a576e",
    "a1d2e6",
    "cad3ff",
    "e9e7e1",
    "eff0f9",
    "f2fbff",
    "ff661a",
    "19f988",
    "ffdc2d",
    "f2e6a9",
    "fb6acb",
    "40bfff",
    "ae94f9",
    "6187ff",
}
