use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SolarflarePalette {
    SolarFlareLight,
    SolarFlare,
}
create_palette! {
    SolarFlareLight,
    "F5F7FA",
    "E8E9ED",
    "A6AFB8",
    "85939E",
    "667581",
    "586875",
    "222E38",
    "18262F",
    "EF5253",
    "E66B2B",
    "E4B51C",
    "7CC844",
    "52CBB0",
    "33B5E1",
    "A363D5",
    "D73C9A",
}
create_palette! {
    SolarFlare,
    "18262F",
    "222E38",
    "586875",
    "667581",
    "85939E",
    "A6AFB8",
    "E8E9ED",
    "F5F7FA",
    "EF5253",
    "E66B2B",
    "E4B51C",
    "7CC844",
    "52CBB0",
    "33B5E1",
    "A363D5",
    "D73C9A",
}
