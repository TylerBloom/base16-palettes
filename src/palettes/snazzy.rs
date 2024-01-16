use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SnazzyPalette {
    Snazzy,
}
create_palette! {
    Snazzy,
    "282a36",
    "34353e",
    "43454f",
    "78787e",
    "a5a5a9",
    "e2e4e5",
    "eff0eb",
    "f1f1f0",
    "ff5c57",
    "ff9f43",
    "f3f99d",
    "5af78e",
    "9aedfe",
    "57c7ff",
    "ff6ac1",
    "b2643c",
}
