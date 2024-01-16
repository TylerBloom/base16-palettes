use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TwilightPalette {
    Twilight,
}
create_palette! {
    Twilight,
    "1e1e1e",
    "323537",
    "464b50",
    "5f5a60",
    "838184",
    "a7a7a7",
    "c3c3c3",
    "ffffff",
    "cf6a4c",
    "cda869",
    "f9ee98",
    "8f9d6a",
    "afc4db",
    "7587a6",
    "9b859d",
    "9b703f",
}
