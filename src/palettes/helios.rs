use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeliosPalette {
    Helios,
}
create_palette! {
    Helios,
    "1d2021",
    "383c3e",
    "53585b",
    "6f7579",
    "cdcdcd",
    "d5d5d5",
    "dddddd",
    "e5e5e5",
    "d72638",
    "eb8413",
    "f19d1a",
    "88b92d",
    "1ba595",
    "1e8bac",
    "be4264",
    "c85e0d",
}
