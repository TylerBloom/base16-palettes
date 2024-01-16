use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SandcastlePalette {
    Sandcastle,
}
create_palette! {
    Sandcastle,
    "282c34",
    "2c323b",
    "3e4451",
    "665c54",
    "928374",
    "a89984",
    "d5c4a1",
    "fdf4c1",
    "83a598",
    "a07e3b",
    "a07e3b",
    "528b8b",
    "83a598",
    "83a598",
    "d75f5f",
    "a87322",
}
