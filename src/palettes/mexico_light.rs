use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MexicoLightPalette {
    MexicoLight,
}
create_palette! {
    MexicoLight,
    "f8f8f8",
    "e8e8e8",
    "d8d8d8",
    "b8b8b8",
    "585858",
    "383838",
    "282828",
    "181818",
    "ab4642",
    "dc9656",
    "f79a0e",
    "538947",
    "4b8093",
    "7cafc2",
    "96609e",
    "a16946",
}
