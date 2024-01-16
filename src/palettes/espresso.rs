use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EspressoPalette {
    Espresso,
    Decaf,
}
create_palette! {
    Espresso,
    "2d2d2d",
    "393939",
    "515151",
    "777777",
    "b4b7b4",
    "cccccc",
    "e0e0e0",
    "ffffff",
    "d25252",
    "f9a959",
    "ffc66d",
    "a5c261",
    "bed6ff",
    "6c99bb",
    "d197d9",
    "f97394",
}
create_palette! {
    Decaf,
    "2d2d2d",
    "393939",
    "515151",
    "777777",
    "b4b7b4",
    "cccccc",
    "e0e0e0",
    "ffffff",
    "ff7f7b",
    "ffbf70",
    "ffd67c",
    "beda78",
    "bed6ff",
    "90bee1",
    "efb3f7",
    "ff93b3",
}
