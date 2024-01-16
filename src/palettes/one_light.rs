use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OneLightPalette {
    OneLight,
}
create_palette! {
    OneLight,
    "fafafa",
    "f0f0f1",
    "e5e5e6",
    "a0a1a7",
    "696c77",
    "383a42",
    "202227",
    "090a0b",
    "ca1243",
    "d75f00",
    "c18401",
    "50a14f",
    "0184bc",
    "4078f2",
    "a626a4",
    "986801",
}
