use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardcorePalette {
    Hardcore,
}
create_palette! {
    Hardcore,
    "212121",
    "303030",
    "353535",
    "4A4A4A",
    "707070",
    "cdcdcd",
    "e5e5e5",
    "ffffff",
    "f92672",
    "fd971f",
    "e6db74",
    "a6e22e",
    "708387",
    "66d9ef",
    "9e6ffe",
    "e8b882",
}
