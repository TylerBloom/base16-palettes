use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QualiaPalette {
    Qualia,
}
create_palette! {
    Qualia,
    "101010",
    "454545",
    "454545",
    "454545",
    "808080",
    "C0C0C0",
    "C0C0C0",
    "454545",
    "EFA6A2",
    "A3B8EF",
    "E6A3DC",
    "80C990",
    "C8C874",
    "50CACD",
    "E0AF85",
    "808080",
}
