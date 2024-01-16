use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SakuraPalette {
    Sakura,
}
create_palette! {
    Sakura,
    "feedf3",
    "f8e2e7",
    "e0ccd1",
    "755f64",
    "665055",
    "564448",
    "42383a",
    "33292b",
    "df2d52",
    "f6661e",
    "c29461",
    "2e916d",
    "1d8991",
    "006e93",
    "5e2180",
    "ba0d35",
}
