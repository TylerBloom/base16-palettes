use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EvaPalette {
    EvaDim,
    Eva,
}
create_palette! {
    EvaDim,
    "2a3b4d",
    "3d566f",
    "4b6988",
    "55799c",
    "7e90a3",
    "9fa2a6",
    "d6d7d9",
    "ffffff",
    "c4676c",
    "ff9966",
    "cfd05d",
    "5de561",
    "4b8f77",
    "1ae1dc",
    "9c6cd3",
    "bb64a9",
}
create_palette! {
    Eva,
    "2a3b4d",
    "3d566f",
    "4b6988",
    "55799c",
    "7e90a3",
    "9fa2a6",
    "d6d7d9",
    "ffffff",
    "c4676c",
    "ff9966",
    "ffff66",
    "66ff66",
    "4b8f77",
    "15f4ee",
    "9c6cd3",
    "bb64a9",
}
