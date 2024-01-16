use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MateriaPalette {
    Materia,
}
create_palette! {
    Materia,
    "263238",
    "2C393F",
    "37474F",
    "707880",
    "C9CCD3",
    "CDD3DE",
    "D5DBE5",
    "FFFFFF",
    "EC5F67",
    "EA9560",
    "FFCC00",
    "8BD649",
    "80CBC4",
    "89DDFF",
    "82AAFF",
    "EC5F67",
}
