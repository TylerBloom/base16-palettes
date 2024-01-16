use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialVividPalette {
    MaterialVivid,
}
create_palette! {
    MaterialVivid,
    "202124",
    "27292c",
    "323639",
    "44464d",
    "676c71",
    "80868b",
    "9e9e9e",
    "ffffff",
    "f44336",
    "ff9800",
    "ffeb3b",
    "00e676",
    "00bcd4",
    "2196f3",
    "673ab7",
    "8d6e63",
}
