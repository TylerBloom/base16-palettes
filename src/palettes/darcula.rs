use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DarculaPalette {
    Darcula,
}
create_palette! {
    Darcula,
    "2b2b2b",
    "323232",
    "323232",
    "606366",
    "a4a3a3",
    "a9b7c6",
    "ffc66d",
    "ffffff",
    "4eade5",
    "689757",
    "bbb529",
    "6a8759",
    "629755",
    "9876aa",
    "cc7832",
    "808080",
}
