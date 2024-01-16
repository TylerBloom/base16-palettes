use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RosePinePalette {
    RoséPine,
    RoséPineMoon,
    RoséPineDawn,
}
create_palette! {
    RoséPine,
    "191724",
    "1f1d2e",
    "26233a",
    "6e6a86",
    "908caa",
    "e0def4",
    "e0def4",
    "524f67",
    "eb6f92",
    "f6c177",
    "ebbcba",
    "31748f",
    "9ccfd8",
    "c4a7e7",
    "f6c177",
    "524f67",
}
create_palette! {
    RoséPineMoon,
    "232136",
    "2a273f",
    "393552",
    "6e6a86",
    "908caa",
    "e0def4",
    "e0def4",
    "56526e",
    "eb6f92",
    "f6c177",
    "ea9a97",
    "3e8fb0",
    "9ccfd8",
    "c4a7e7",
    "f6c177",
    "56526e",
}
create_palette! {
    RoséPineDawn,
    "faf4ed",
    "fffaf3",
    "f2e9de",
    "9893a5",
    "797593",
    "575279",
    "575279",
    "cecacd",
    "b4637a",
    "ea9d34",
    "d7827e",
    "286983",
    "56949f",
    "907aa9",
    "ea9d34",
    "cecacd",
}
