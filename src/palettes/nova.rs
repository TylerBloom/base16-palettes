use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NovaPalette {
    Nova,
}
create_palette! {
    Nova,
    "3C4C55",
    "556873",
    "6A7D89",
    "899BA6",
    "899BA6",
    "C5D4DD",
    "899BA6",
    "556873",
    "83AFE5",
    "7FC1CA",
    "A8CE93",
    "7FC1CA",
    "F2C38F",
    "83AFE5",
    "9A93E1",
    "F2C38F",
}
