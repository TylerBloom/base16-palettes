use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GithubPalette {
    Github,
}
create_palette! {
    Github,
    "ffffff",
    "f5f5f5",
    "c8c8fa",
    "969896",
    "e8e8e8",
    "333333",
    "ffffff",
    "ffffff",
    "ed6a43",
    "0086b3",
    "795da3",
    "183691",
    "183691",
    "795da3",
    "a71d5d",
    "333333",
}
