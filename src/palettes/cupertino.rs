use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CupertinoPalette {
    Cupertino,
}
create_palette! {
    Cupertino,
    "ffffff",
    "c0c0c0",
    "c0c0c0",
    "808080",
    "808080",
    "404040",
    "404040",
    "5e5e5e",
    "c41a15",
    "eb8500",
    "826b28",
    "007400",
    "318495",
    "0000ff",
    "a90d91",
    "826b28",
}
