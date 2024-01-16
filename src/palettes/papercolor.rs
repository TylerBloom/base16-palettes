use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PapercolorPalette {
    PaperColorDark,
    PaperColorLight,
}
create_palette! {
    PaperColorDark,
    "1c1c1c",
    "af005f",
    "5faf00",
    "d7af5f",
    "5fafd7",
    "808080",
    "d7875f",
    "d0d0d0",
    "585858",
    "5faf5f",
    "afd700",
    "af87d7",
    "ffaf00",
    "ff5faf",
    "00afaf",
    "5f8787",
}
create_palette! {
    PaperColorLight,
    "eeeeee",
    "af0000",
    "008700",
    "5f8700",
    "0087af",
    "444444",
    "005f87",
    "878787",
    "bcbcbc",
    "d70000",
    "d70087",
    "8700af",
    "d75f00",
    "d75f00",
    "005faf",
    "005f87",
}
