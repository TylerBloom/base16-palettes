use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MaterialPalette {
    MaterialPalenight,
    Material,
    MaterialDarker,
    MaterialLighter,
}
create_palette! {
    MaterialPalenight,
    "292D3E",
    "444267",
    "32374D",
    "676E95",
    "8796B0",
    "959DCB",
    "959DCB",
    "FFFFFF",
    "F07178",
    "F78C6C",
    "FFCB6B",
    "C3E88D",
    "89DDFF",
    "82AAFF",
    "C792EA",
    "FF5370",
}
create_palette! {
    Material,
    "263238",
    "2E3C43",
    "314549",
    "546E7A",
    "B2CCD6",
    "EEFFFF",
    "EEFFFF",
    "FFFFFF",
    "F07178",
    "F78C6C",
    "FFCB6B",
    "C3E88D",
    "89DDFF",
    "82AAFF",
    "C792EA",
    "FF5370",
}
create_palette! {
    MaterialDarker,
    "212121",
    "303030",
    "353535",
    "4A4A4A",
    "B2CCD6",
    "EEFFFF",
    "EEFFFF",
    "FFFFFF",
    "F07178",
    "F78C6C",
    "FFCB6B",
    "C3E88D",
    "89DDFF",
    "82AAFF",
    "C792EA",
    "FF5370",
}
create_palette! {
    MaterialLighter,
    "FAFAFA",
    "E7EAEC",
    "CCEAE7",
    "CCD7DA",
    "8796B0",
    "80CBC4",
    "80CBC4",
    "FFFFFF",
    "FF5370",
    "F76D47",
    "FFB62C",
    "91B859",
    "39ADB5",
    "6182B8",
    "7C4DFF",
    "E53935",
}