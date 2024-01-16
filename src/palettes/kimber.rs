use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KimberPalette {
    Kimber,
}
create_palette! {
    Kimber,
    "222222",
    "313131",
    "555D55",
    "644646",
    "5A5A5A",
    "DEDEE7",
    "C3C3B4",
    "FFFFE6",
    "C88C8C",
    "476C88",
    "D8B56D",
    "99C899",
    "78B4B4",
    "537C9C",
    "86CACD",
    "704F4F",
}
