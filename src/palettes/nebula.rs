use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NebulaPalette {
    Nebula,
}
create_palette! {
    Nebula,
    "22273b",
    "414f60",
    "5a8380",
    "6e6f72",
    "87888b",
    "a4a6a9",
    "c7c9cd",
    "8dbdaa",
    "777abc",
    "94929e",
    "4f9062",
    "6562a8",
    "226f68",
    "4d6bb6",
    "716cae",
    "8c70a7",
}
