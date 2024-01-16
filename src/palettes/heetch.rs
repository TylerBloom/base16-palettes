use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HeetchPalette {
    HeetchLight,
    HeetchDark,
}
create_palette! {
    HeetchLight,
    "feffff",
    "392551",
    "7b6d8b",
    "9c92a8",
    "ddd6e5",
    "5a496e",
    "470546",
    "190134",
    "27d9d5",
    "bdb6c5",
    "5ba2b6",
    "f80059",
    "c33678",
    "47f9f5",
    "bd0152",
    "dedae2",
}
create_palette! {
    HeetchDark,
    "190134",
    "392551",
    "5A496E",
    "7B6D8B",
    "9C92A8",
    "BDB6C5",
    "DEDAE2",
    "FEFFFF",
    "27D9D5",
    "5BA2B6",
    "8F6C97",
    "C33678",
    "F80059",
    "BD0152",
    "82034C",
    "470546",
}
