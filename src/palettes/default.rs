use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DefaultPalette {
    Cupcake,
    DefaultDark,
    DefaultLight,
    Eighties,
    Mocha,
    Ocean,
}

impl Default for DefaultPalette {
    fn default() -> Self {
        Self::DefaultDark(DefaultDark)
    }
}

create_palette! {
    Cupcake,
    "fbf1f2",
    "f2f1f4",
    "d8d5dd",
    "bfb9c6",
    "a59daf",
    "8b8198",
    "72677E",
    "585062",
    "D57E85",
    "EBB790",
    "DCB16C",
    "A3B367",
    "69A9A7",
    "7297B9",
    "BB99B4",
    "BAA58C",
}

create_palette! {
    DefaultDark,
    "181818",
    "282828",
    "383838",
    "585858",
    "b8b8b8",
    "d8d8d8",
    "e8e8e8",
    "f8f8f8",
    "ab4642",
    "dc9656",
    "f7ca88",
    "a1b56c",
    "86c1b9",
    "7cafc2",
    "ba8baf",
    "a16946",
}

create_palette! {
    DefaultLight,
    "f8f8f8",
    "e8e8e8",
    "d8d8d8",
    "b8b8b8",
    "585858",
    "383838",
    "282828",
    "181818",
    "ab4642",
    "dc9656",
    "f7ca88",
    "a1b56c",
    "86c1b9",
    "7cafc2",
    "ba8baf",
    "a16946",
}

create_palette! {
    Eighties,
    "2d2d2d",
    "393939",
    "515151",
    "747369",
    "a09f93",
    "d3d0c8",
    "e8e6df",
    "f2f0ec",
    "f2777a",
    "f99157",
    "ffcc66",
    "99cc99",
    "66cccc",
    "6699cc",
    "cc99cc",
    "d27b53",
}

create_palette! {
    Mocha,
    "3B3228",
    "534636",
    "645240",
    "7e705a",
    "b8afad",
    "d0c8c6",
    "e9e1dd",
    "f5eeeb",
    "cb6077",
    "d28b71",
    "f4bc87",
    "beb55b",
    "7bbda4",
    "8ab3b5",
    "a89bb9",
    "bb9584",
}

create_palette! {
    Ocean,
    "2b303b",
    "343d46",
    "4f5b66",
    "65737e",
    "a7adba",
    "c0c5ce",
    "dfe1e8",
    "eff1f5",
    "bf616a",
    "d08770",
    "ebcb8b",
    "a3be8c",
    "96b5b4",
    "8fa1b3",
    "b48ead",
    "ab7967",
}
