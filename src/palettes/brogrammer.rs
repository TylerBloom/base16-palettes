use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BrogrammerPalette {
    Brogrammer,
}
create_palette! {
    Brogrammer,
    "1f1f1f",
    "f81118",
    "2dc55e",
    "ecba0f",
    "2a84d2",
    "4e5ab7",
    "1081d6",
    "d6dbe5",
    "d6dbe5",
    "de352e",
    "1dd361",
    "f3bd09",
    "1081d6",
    "5350b9",
    "0f7ddb",
    "ffffff",
}
