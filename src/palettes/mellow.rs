use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MellowPalette {
    MellowPurple,
}
create_palette! {
    MellowPurple,
    "1e0528",
    "1A092D",
    "331354",
    "320f55",
    "873582",
    "ffeeff",
    "ffeeff",
    "f8c0ff",
    "00d9e9",
    "aa00a3",
    "955ae7",
    "05cb0d",
    "b900b1",
    "550068",
    "8991bb",
    "4d6fff",
}
