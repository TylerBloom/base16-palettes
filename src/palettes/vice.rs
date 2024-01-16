use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VicePalette {
    ViceDark,
    ViceAlt,
}
create_palette! {
    ViceDark,
    "181818",
    "222222",
    "323232",
    "3f3f3f",
    "666666",
    "818181",
    "c6c6c6",
    "e9e9e9",
    "ff29a8",
    "85ffe0",
    "f0ffaa",
    "0badff",
    "8265ff",
    "00eaff",
    "00f6d9",
    "ff3d81",
}
create_palette! {
    ViceAlt,
    "1c1c1c",
    "282828",
    "2c2c2c",
    "323232",
    "3c3c3c",
    "555555",
    "b6b6b6",
    "d1d1d1",
    "ff3d81",
    "F67544",
    "ffff73",
    "44ffdd",
    "00caff",
    "2fb1d4",
    "8265ff",
    "F83D80",
}
