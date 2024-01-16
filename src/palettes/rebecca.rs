use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RebeccaPalette {
    Rebecca,
}
create_palette! {
    Rebecca,
    "292a44",
    "663399",
    "383a62",
    "666699",
    "a0a0c5",
    "f1eff8",
    "ccccff",
    "53495d",
    "a0a0c5",
    "efe4a1",
    "ae81ff",
    "6dfedf",
    "8eaee0",
    "2de0a7",
    "7aa5ff",
    "ff79c6",
}
