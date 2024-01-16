use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IcyPalette {
    IcyDark,
}
create_palette! {
    IcyDark,
    "021012",
    "031619",
    "041f23",
    "052e34",
    "064048",
    "095b67",
    "0c7c8c",
    "109cb0",
    "16c1d9",
    "b3ebf2",
    "80deea",
    "4dd0e1",
    "26c6da",
    "00bcd4",
    "00acc1",
    "0097a7",
}
