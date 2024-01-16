/// This module contains the color palettes that are supported by [base16](). These can be
/// converted into styles used by Ratatui.
pub mod default;
pub use default::*;

// TODO: Only a few palettes are implemented.
//  - Except for Grubbox, everything past the "a"s is not done
#[cfg(feature = "apprentice")]
pub mod apprentice;
#[cfg(feature = "apprentice")]
pub use apprentice::*;
#[cfg(feature = "atelier")]
pub mod atelier;
#[cfg(feature = "atelier")]
pub use atelier::*;
#[cfg(feature = "atlas")]
pub mod atlas;
#[cfg(feature = "atlas")]
pub use atlas::*;
#[cfg(feature = "black_metal")]
pub mod black_metal;
#[cfg(feature = "brogrammer")]
pub mod brogrammer;
#[cfg(feature = "brush_trees")]
pub mod brush_trees;
#[cfg(feature = "circus")]
pub mod circus;
#[cfg(feature = "classic")]
pub mod classic;
#[cfg(feature = "codeschool")]
pub mod codeschool;
#[cfg(feature = "colors")]
pub mod colors;
#[cfg(feature = "cupertino")]
pub mod cupertino;
#[cfg(feature = "danqing")]
pub mod danqing;
#[cfg(feature = "darcula")]
pub mod darcula;
#[cfg(feature = "darkviolet")]
pub mod darkviolet;
#[cfg(feature = "dracula")]
pub mod dracula;
#[cfg(feature = "equilibrium")]
pub mod equilibrium;
#[cfg(feature = "espresso")]
pub mod espresso;
#[cfg(feature = "eva")]
pub mod eva;
#[cfg(feature = "framer")]
pub mod framer;
#[cfg(feature = "fruit_soda")]
pub mod fruit_soda;
#[cfg(feature = "gigavolt")]
pub mod gigavolt;
#[cfg(feature = "github")]
pub mod github;
#[cfg(feature = "gruvbox")]
pub mod gruvbox;
#[cfg(feature = "gruvbox")]
pub use gruvbox::*;
#[cfg(feature = "hardcore")]
pub mod hardcore;
#[cfg(feature = "heetch")]
pub mod heetch;
#[cfg(feature = "helios")]
pub mod helios;
#[cfg(feature = "horizon")]
pub mod horizon;
#[cfg(feature = "humanoid")]
pub mod humanoid;
#[cfg(feature = "ia")]
pub mod ia;
#[cfg(feature = "icy")]
pub mod icy;
#[cfg(feature = "kimber")]
pub mod kimber;
#[cfg(feature = "materia")]
pub mod materia;
#[cfg(feature = "material_theme")]
pub mod material_theme;
#[cfg(feature = "material_vivid")]
pub mod material_vivid;
#[cfg(feature = "mellow")]
pub mod mellow;
#[cfg(feature = "mexico_light")]
pub mod mexico_light;
#[cfg(feature = "nebula")]
pub mod nebula;
#[cfg(feature = "nord")]
pub mod nord;
#[cfg(feature = "nova")]
pub mod nova;
#[cfg(feature = "one_light")]
pub mod one_light;
#[cfg(feature = "onedark")]
pub mod onedark;
#[cfg(feature = "outrun")]
pub mod outrun;
#[cfg(feature = "papercolor")]
pub mod papercolor;
#[cfg(feature = "pasque")]
pub mod pasque;
#[cfg(feature = "pinky")]
pub mod pinky;
#[cfg(feature = "porple")]
pub mod porple;
#[cfg(feature = "purpledream")]
pub mod purpledream;
#[cfg(feature = "qualia")]
pub mod qualia;
#[cfg(feature = "rebecca")]
pub mod rebecca;
#[cfg(feature = "rose_pine")]
pub mod rose_pine;
#[cfg(feature = "sagelight")]
pub mod sagelight;
#[cfg(feature = "sakura")]
pub mod sakura;
#[cfg(feature = "sandcastle")]
pub mod sandcastle;
#[cfg(feature = "shades_of_purple")]
pub mod shades_of_purple;
#[cfg(feature = "silk")]
pub mod silk;
#[cfg(feature = "snazzy")]
pub mod snazzy;
#[cfg(feature = "solarflare")]
pub mod solarflare;
#[cfg(feature = "solarized")]
pub mod solarized;
#[cfg(feature = "summercamp")]
pub mod summercamp;
#[cfg(feature = "summerfruit")]
pub mod summerfruit;
#[cfg(feature = "synth_midnight")]
pub mod synth_midnight;
#[cfg(feature = "tender")]
pub mod tender;
#[cfg(feature = "twilight")]
pub mod twilight;
#[cfg(feature = "unikitty")]
pub mod unikitty;
#[cfg(feature = "vice")]
pub mod vice;
#[cfg(feature = "windows")]
pub mod windows;
#[cfg(feature = "woodland")]
pub mod woodland;
#[cfg(feature = "xcode_dust")]
pub mod xcode_dust;
#[cfg(feature = "zenburn")]
pub mod zenburn;

pub(crate) use create::create_palette;

mod create {
    macro_rules! create_palette {
        ($name:ident,
        $s01:literal,
        $s02:literal,
        $s03:literal,
        $s04:literal,
        $s05:literal,
        $s06:literal,
        $s07:literal,
        $s08:literal,
        $s09:literal,
        $s10:literal,
        $s11:literal,
        $s12:literal,
        $s13:literal,
        $s14:literal,
        $s15:literal,
        $s16:literal,
        ) => {
            #[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
            pub struct $name;

            impl crate::Base16Palette for $name {
                fn to_rgb(&self, color: crate::Base16Color) -> (u8, u8, u8) {
                    match color {
                        crate::Base16Color::Shade(crate::Base16Shade::Dark(
                            crate::Shade::Darkest,
                        )) => hex_literal::hex!($s01).into(),
                        crate::Base16Color::Shade(crate::Base16Shade::Dark(
                            crate::Shade::Darker,
                        )) => hex_literal::hex!($s02).into(),
                        crate::Base16Color::Shade(crate::Base16Shade::Dark(
                            crate::Shade::Lighter,
                        )) => hex_literal::hex!($s03).into(),
                        crate::Base16Color::Shade(crate::Base16Shade::Dark(
                            crate::Shade::Lightest,
                        )) => hex_literal::hex!($s04).into(),
                        crate::Base16Color::Shade(crate::Base16Shade::Light(
                            crate::Shade::Darkest,
                        )) => hex_literal::hex!($s05).into(),
                        crate::Base16Color::Shade(crate::Base16Shade::Light(
                            crate::Shade::Darker,
                        )) => hex_literal::hex!($s06).into(),
                        crate::Base16Color::Shade(crate::Base16Shade::Light(
                            crate::Shade::Lighter,
                        )) => hex_literal::hex!($s07).into(),
                        crate::Base16Color::Shade(crate::Base16Shade::Light(
                            crate::Shade::Lightest,
                        )) => hex_literal::hex!($s08).into(),
                        crate::Base16Color::Accent(crate::Base16Accent::Accent00) => {
                            hex_literal::hex!($s09).into()
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent01) => {
                            hex_literal::hex!($s10).into()
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent02) => {
                            hex_literal::hex!($s11).into()
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent03) => {
                            hex_literal::hex!($s12).into()
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent04) => {
                            hex_literal::hex!($s13).into()
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent05) => {
                            hex_literal::hex!($s14).into()
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent06) => {
                            hex_literal::hex!($s15).into()
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent07) => {
                            hex_literal::hex!($s16).into()
                        }
                    }
                }

                fn to_hex_str(&self, color: crate::Base16Color) -> &'static str {
                    match color {
                        crate::Base16Color::Shade(crate::Base16Shade::Dark(
                            crate::Shade::Darkest,
                        )) => std::concat!("#", $s01),
                        crate::Base16Color::Shade(crate::Base16Shade::Dark(
                            crate::Shade::Darker,
                        )) => std::concat!("#", $s02),
                        crate::Base16Color::Shade(crate::Base16Shade::Dark(
                            crate::Shade::Lighter,
                        )) => std::concat!("#", $s03),
                        crate::Base16Color::Shade(crate::Base16Shade::Dark(
                            crate::Shade::Lightest,
                        )) => std::concat!("#", $s04),
                        crate::Base16Color::Shade(crate::Base16Shade::Light(
                            crate::Shade::Darkest,
                        )) => std::concat!("#", $s05),
                        crate::Base16Color::Shade(crate::Base16Shade::Light(
                            crate::Shade::Darker,
                        )) => std::concat!("#", $s06),
                        crate::Base16Color::Shade(crate::Base16Shade::Light(
                            crate::Shade::Lighter,
                        )) => std::concat!("#", $s07),
                        crate::Base16Color::Shade(crate::Base16Shade::Light(
                            crate::Shade::Lightest,
                        )) => std::concat!("#", $s08),
                        crate::Base16Color::Accent(crate::Base16Accent::Accent00) => {
                            std::concat!("#", $s09)
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent01) => {
                            std::concat!("#", $s10)
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent02) => {
                            std::concat!("#", $s11)
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent03) => {
                            std::concat!("#", $s12)
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent04) => {
                            std::concat!("#", $s13)
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent05) => {
                            std::concat!("#", $s14)
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent06) => {
                            std::concat!("#", $s15)
                        }
                        crate::Base16Color::Accent(crate::Base16Accent::Accent07) => {
                            std::concat!("#", $s16)
                        }
                    }
                }
            }
        };
    }

    pub(crate) use create_palette;
}
