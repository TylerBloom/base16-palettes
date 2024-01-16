use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SagelightPalette {
    Sagelight,
}
create_palette! {
    Sagelight,
    "f8f8f8",
    "e8e8e8",
    "d8d8d8",
    "b8b8b8",
    "585858",
    "383838",
    "282828",
    "181818",
    "fa8480",
    "ffaa61",
    "ffdc61",
    "a0d2c8",
    "a2d6f5",
    "a0a7d2",
    "c8a0d2",
    "d2b2a0",
}
