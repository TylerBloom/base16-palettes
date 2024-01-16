use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraculaPalette {
    Dracula,
}
create_palette! {
    Dracula,
    "282936",
    "3a3c4e",
    "4d4f68",
    "626483",
    "62d6e8",
    "e9e9f4",
    "f1f2f8",
    "f7f7fb",
    "ea51b2",
    "b45bcf",
    "00f769",
    "ebff87",
    "a1efe4",
    "62d6e8",
    "b45bcf",
    "00f769",
}
