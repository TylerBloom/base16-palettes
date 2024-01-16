use enum_dispatch::enum_dispatch;

use super::create_palette;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SynthMidnightPalette {
    SynthMidnightTerminalLight,
    SynthMidnightTerminalDark,
}
create_palette! {
    SynthMidnightTerminalLight,
    "dddfe0",
    "cfd1d2",
    "c1c3c4",
    "a3a5a6",
    "474849",
    "28292a",
    "1a1b1c",
    "050608",
    "b53b50",
    "ea770d",
    "c9d364",
    "06ea61",
    "42fff9",
    "03aeff",
    "ea5ce2",
    "cd6320",
}
create_palette! {
    SynthMidnightTerminalDark,
    "050608",
    "1a1b1c",
    "28292a",
    "474849",
    "a3a5a6",
    "c1c3c4",
    "cfd1d2",
    "dddfe0",
    "b53b50",
    "ea770d",
    "c9d364",
    "06ea61",
    "42fff9",
    "03aeff",
    "ea5ce2",
    "cd6320",
}
