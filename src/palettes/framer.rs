use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FramerPalette {
    Framer,
}
create_palette! {
    Framer,
    "181818",
    "151515",
    "464646",
    "747474",
    "B9B9B9",
    "D0D0D0",
    "E8E8E8",
    "EEEEEE",
    "FD886B",
    "FC4769",
    "FECB6E",
    "32CCDC",
    "ACDDFD",
    "20BCFC",
    "BA8CFC",
    "B15F4A",
}
