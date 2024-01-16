use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ZenburnPalette {
    Zenburn,
}
create_palette! {
    Zenburn,
    "383838",
    "404040",
    "606060",
    "6f6f6f",
    "808080",
    "dcdccc",
    "c0c0c0",
    "ffffff",
    "dca3a3",
    "dfaf8f",
    "e0cf9f",
    "5f7f5f",
    "93e0e3",
    "7cb8bb",
    "dc8cc3",
    "000000",
}
