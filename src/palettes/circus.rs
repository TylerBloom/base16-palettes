use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CircusPalette {
    Circus,
}
create_palette! {
    Circus,
    "191919",
    "202020",
    "303030",
    "5f5a60",
    "505050",
    "a7a7a7",
    "808080",
    "ffffff",
    "dc657d",
    "4bb1a7",
    "c3ba63",
    "84b97c",
    "4bb1a7",
    "639ee4",
    "b888e2",
    "b888e2",
}
