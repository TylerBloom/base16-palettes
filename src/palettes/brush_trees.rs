use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrushTreesPalette {
    BrushTrees,
    BrushTreesDark,
}
create_palette! {
    BrushTrees,
    "E3EFEF",
    "C9DBDC",
    "B0C5C8",
    "98AFB5",
    "8299A1",
    "6D828E",
    "5A6D7A",
    "485867",
    "b38686",
    "d8bba2",
    "aab386",
    "87b386",
    "86b3b3",
    "868cb3",
    "b386b2",
    "b39f9f",
}
create_palette! {
    BrushTreesDark,
    "485867",
    "5A6D7A",
    "6D828E",
    "8299A1",
    "98AFB5",
    "B0C5C8",
    "C9DBDC",
    "E3EFEF",
    "b38686",
    "d8bba2",
    "aab386",
    "87b386",
    "86b3b3",
    "868cb3",
    "b386b2",
    "b39f9f",
}
