use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OnedarkPalette {
    OneDark,
}
create_palette! {
    OneDark,
    "282c34",
    "353b45",
    "3e4451",
    "545862",
    "565c64",
    "abb2bf",
    "b6bdca",
    "c8ccd4",
    "e06c75",
    "d19a66",
    "e5c07b",
    "98c379",
    "56b6c2",
    "61afef",
    "c678dd",
    "be5046",
}
