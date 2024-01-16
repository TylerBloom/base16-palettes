use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodeschoolPalette {
    Codeschool,
}
create_palette! {
    Codeschool,
    "232c31",
    "1c3657",
    "2a343a",
    "3f4944",
    "84898c",
    "9ea7a6",
    "a7cfa3",
    "b5d8f6",
    "2a5491",
    "43820d",
    "a03b1e",
    "237986",
    "b02f30",
    "484d79",
    "c59820",
    "c98344",
}
