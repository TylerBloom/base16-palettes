use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NordPalette {
    Nord,
}
create_palette! {
    Nord,
    "2E3440",
    "3B4252",
    "434C5E",
    "4C566A",
    "D8DEE9",
    "E5E9F0",
    "ECEFF4",
    "8FBCBB",
    "88C0D0",
    "81A1C1",
    "5E81AC",
    "BF616A",
    "D08770",
    "EBCB8B",
    "A3BE8C",
    "B48EAD",
}
