use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TenderPalette {
    Tender,
}
create_palette! {
    Tender,
    "282828",
    "383838",
    "484848",
    "4c4c4c",
    "b8b8b8",
    "eeeeee",
    "e8e8e8",
    "feffff",
    "f43753",
    "dc9656",
    "ffc24b",
    "c9d05c",
    "73cef4",
    "b3deef",
    "d3b987",
    "a16946",
}
