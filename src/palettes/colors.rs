use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorsPalette {
    Colors,
}
create_palette! {
    Colors,
    "111111",
    "333333",
    "555555",
    "777777",
    "999999",
    "bbbbbb",
    "dddddd",
    "ffffff",
    "ff4136",
    "ff851b",
    "ffdc00",
    "2ecc40",
    "7fdbff",
    "0074d9",
    "b10dc9",
    "85144b",
}
