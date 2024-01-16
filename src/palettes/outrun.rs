use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutrunPalette {
    OutrunDark,
}
create_palette! {
    OutrunDark,
    "00002A",
    "20204A",
    "30305A",
    "50507A",
    "B0B0DA",
    "D0D0FA",
    "E0E0FF",
    "F5F5FF",
    "FF4242",
    "FC8D28",
    "F3E877",
    "59F176",
    "0EF0F0",
    "66B0FF",
    "F10596",
    "F003EF",
}
