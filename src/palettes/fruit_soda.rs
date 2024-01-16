use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FruitSodaPalette {
    FruitSoda,
}
create_palette! {
    FruitSoda,
    "f1ecf1",
    "e0dee0",
    "d8d5d5",
    "b5b4b6",
    "979598",
    "515151",
    "474545",
    "2d2c2c",
    "fe3e31",
    "fe6d08",
    "f7e203",
    "47f74c",
    "0f9cfd",
    "2931df",
    "611fce",
    "b16f40",
}
