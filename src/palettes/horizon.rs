use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HorizonPalette {
    HorizonDark,
    HorizonLight,
    HorizonTerminalLight,
    HorizonTerminalDark,
}
create_palette! {
    HorizonDark,
    "1C1E26",
    "232530",
    "2E303E",
    "6F6F70",
    "9DA0A2",
    "CBCED0",
    "DCDFE4",
    "E3E6EE",
    "E95678",
    "FAB795",
    "FAC29A",
    "29D398",
    "59E1E3",
    "26BBD9",
    "EE64AC",
    "F09383",
}
create_palette! {
    HorizonLight,
    "FDF0ED",
    "FADAD1",
    "F9CBBE",
    "BDB3B1",
    "948C8A",
    "403C3D",
    "302C2D",
    "201C1D",
    "E95678",
    "F9CEC3",
    "FADAD1",
    "29D398",
    "59E1E3",
    "26BBD9",
    "EE64AC",
    "F9CBBE",
}
create_palette! {
    HorizonTerminalLight,
    "FDF0ED",
    "FADAD1",
    "F9CBBE",
    "BDB3B1",
    "948C8A",
    "403C3D",
    "302C2D",
    "201C1D",
    "F7939B",
    "F6661E",
    "FBE0D9",
    "94E1B0",
    "DC3318",
    "DA103F",
    "1D8991",
    "E58C92",
}
create_palette! {
    HorizonTerminalDark,
    "1C1E26",
    "232530",
    "2E303E",
    "6F6F70",
    "9DA0A2",
    "CBCED0",
    "DCDFE4",
    "E3E6EE",
    "E93C58",
    "E58D7D",
    "EFB993",
    "EFAF8E",
    "24A8B4",
    "DF5273",
    "B072D1",
    "E4A382",
}
