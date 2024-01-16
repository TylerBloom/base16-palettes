use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DanqingPalette {
    DanQing,
    DanQingLight,
}
create_palette! {
    DanQing,
    "2d302f",
    "434846",
    "5a605d",
    "9da8a3",
    "cad8d2",
    "e0f0eF",
    "ecf6f2",
    "fcfefd",
    "F9906F",
    "B38A61",
    "F0C239",
    "8AB361",
    "30DFF3",
    "B0A4E3",
    "CCA4E3",
    "CA6924",
}
create_palette! {
    DanQingLight,
    "fcfefd",
    "ecf6f2",
    "e0f0eF",
    "cad8d2",
    "9da8a3",
    "5a605d",
    "434846",
    "2d302f",
    "F9906F",
    "B38A61",
    "F0C239",
    "8AB361",
    "30DFF3",
    "B0A4E3",
    "CCA4E3",
    "CA6924",
}
