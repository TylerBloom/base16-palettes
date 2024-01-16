use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HumanoidPalette {
    HumanoidLight,
    HumanoidDark,
}
create_palette! {
    HumanoidLight,
    "f8f8f2",
    "efefe9",
    "deded8",
    "c0c0bd",
    "60615d",
    "232629",
    "2f3337",
    "070708",
    "b0151a",
    "ff3d00",
    "ffb627",
    "388e3c",
    "008e8e",
    "0082c9",
    "700f98",
    "b27701",
}
create_palette! {
    HumanoidDark,
    "232629",
    "333b3d",
    "484e54",
    "60615d",
    "c0c0bd",
    "f8f8f2",
    "fcfcf6",
    "fcfcfc",
    "f11235",
    "ff9505",
    "ffb627",
    "02d849",
    "0dd9d6",
    "00a6fb",
    "f15ee3",
    "b27701",
}
