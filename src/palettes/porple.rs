use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PorplePalette {
    Porple,
}
create_palette! {
    Porple,
    "292c36",
    "333344",
    "474160",
    "65568a",
    "b8b8b8",
    "d8d8d8",
    "e8e8e8",
    "f8f8f8",
    "f84547",
    "d28e5d",
    "efa16b",
    "95c76f",
    "64878f",
    "8485ce",
    "b74989",
    "986841",
}
