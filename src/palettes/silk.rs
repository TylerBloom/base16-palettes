use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SilkPalette {
    SilkLight,
    SilkDark,
}
create_palette! {
    SilkLight,
    "E9F1EF",
    "CCD4D3",
    "90B7B6",
    "5C787B",
    "4B5B5F",
    "385156",
    "0e3c46",
    "D2FAFF",
    "CF432E",
    "D27F46",
    "CFAD25",
    "6CA38C",
    "329CA2",
    "39AAC9",
    "6E6582",
    "865369",
}
create_palette! {
    SilkDark,
    "0e3c46",
    "1D494E",
    "2A5054",
    "587073",
    "9DC8CD",
    "C7DBDD",
    "CBF2F7",
    "D2FAFF",
    "fb6953",
    "fcab74",
    "fce380",
    "73d8ad",
    "3fb2b9",
    "46bddd",
    "756b8a",
    "9b647b",
}
