use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PinkyPalette {
    Pinky,
}
create_palette! {
    Pinky,
    "171517",
    "1b181b",
    "1d1b1d",
    "383338",
    "e7dbdb",
    "f5f5f5",
    "ffffff",
    "f7f3f7",
    "ffa600",
    "00ff66",
    "20df6c",
    "ff0066",
    "6600ff",
    "00ffff",
    "007fff",
    "df206c",
}
