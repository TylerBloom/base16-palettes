use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XcodeDuskPalette {
    XCodeDusk,
}
create_palette! {
    XCodeDusk,
    "282B35",
    "3D4048",
    "53555D",
    "686A71",
    "7E8086",
    "939599",
    "A9AAAE",
    "BEBFC2",
    "B21889",
    "786DC5",
    "438288",
    "DF0002",
    "00A0BE",
    "790EAD",
    "B21889",
    "C77C48",
}
