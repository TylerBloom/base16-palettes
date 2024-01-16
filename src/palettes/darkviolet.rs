use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DarkvioletPalette {
    DarkViolet,
}
create_palette! {
    DarkViolet,
    "000000",
    "231a40",
    "432d59",
    "593380",
    "00ff00",
    "b08ae6",
    "9045e6",
    "a366ff",
    "a82ee6",
    "bb66cc",
    "f29df2",
    "4595e6",
    "40dfff",
    "4136d9",
    "7e5ce6",
    "a886bf",
}
