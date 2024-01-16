use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasquePalette {
    Pasque,
}
create_palette! {
    Pasque,
    "271C3A",
    "100323",
    "3E2D5C",
    "5D5766",
    "BEBCBF",
    "DEDCDF",
    "EDEAEF",
    "BBAADD",
    "A92258",
    "918889",
    "804ead",
    "C6914B",
    "7263AA",
    "8E7DC6",
    "953B9D",
    "59325C",
}
