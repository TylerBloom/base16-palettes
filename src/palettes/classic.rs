use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClassicPalette {
    ClassicDark,
    ClassicLight,
}
create_palette! {
    ClassicDark,
    "151515",
    "202020",
    "303030",
    "505050",
    "B0B0B0",
    "D0D0D0",
    "E0E0E0",
    "F5F5F5",
    "AC4142",
    "D28445",
    "F4BF75",
    "90A959",
    "75B5AA",
    "6A9FB5",
    "AA759F",
    "8F5536",
}
create_palette! {
    ClassicLight,
    "F5F5F5",
    "E0E0E0",
    "D0D0D0",
    "B0B0B0",
    "505050",
    "303030",
    "202020",
    "151515",
    "AC4142",
    "D28445",
    "F4BF75",
    "90A959",
    "75B5AA",
    "6A9FB5",
    "AA759F",
    "8F5536",
}
