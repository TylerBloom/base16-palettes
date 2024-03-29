use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SummercampPalette {
    Summercamp,
}
create_palette! {
    Summercamp,
    "1c1810",
    "2a261c",
    "3a3527",
    "504b38",
    "5f5b45",
    "736e55",
    "bab696",
    "f8f5de",
    "e35142",
    "fba11b",
    "f2ff27",
    "5ceb5a",
    "5aebbc",
    "489bf0",
    "FF8080",
    "F69BE7",
}
