/// This module contains the color palettes that are supported by [base16](). These can be
/// converted into styles used by Ratatui.
mod default;
pub use default::*;

#[cfg(feature = "apprentice")]
mod apprentice;
#[cfg(feature = "apprentice")]
pub use apprentice::*;
#[cfg(feature = "atelier")]
mod atelier;
#[cfg(feature = "atelier")]
pub use atelier::*;
#[cfg(feature = "atlas")]
mod atlas;
#[cfg(feature = "atlas")]
pub use atlas::*;
#[cfg(feature = "black_metal")]
mod black_metal;
#[cfg(feature = "black_metal")]
pub use black_metal::*;
#[cfg(feature = "brogrammer")]
mod brogrammer;
#[cfg(feature = "brogrammer")]
pub use brogrammer::*;
#[cfg(feature = "brush_trees")]
mod brush_trees;
#[cfg(feature = "brush_trees")]
pub use brush_trees::*;
#[cfg(feature = "circus")]
mod circus;
#[cfg(feature = "circus")]
pub use circus::*;
#[cfg(feature = "classic")]
mod classic;
#[cfg(feature = "classic")]
pub use classic::*;
#[cfg(feature = "codeschool")]
mod codeschool;
#[cfg(feature = "codeschool")]
pub use codeschool::*;
#[cfg(feature = "colors")]
mod colors;
#[cfg(feature = "colors")]
pub use colors::*;
#[cfg(feature = "cupertino")]
mod cupertino;
#[cfg(feature = "cupertino")]
pub use cupertino::*;
#[cfg(feature = "danqing")]
mod danqing;
#[cfg(feature = "danqing")]
pub use danqing::*;
#[cfg(feature = "darcula")]
mod darcula;
#[cfg(feature = "darcula")]
pub use darcula::*;
#[cfg(feature = "darkviolet")]
mod darkviolet;
#[cfg(feature = "darkviolet")]
pub use darkviolet::*;
#[cfg(feature = "dracula")]
mod dracula;
#[cfg(feature = "dracula")]
pub use dracula::*;
#[cfg(feature = "equilibrium")]
mod equilibrium;
#[cfg(feature = "equilibrium")]
pub use equilibrium::*;
#[cfg(feature = "espresso")]
mod espresso;
#[cfg(feature = "espresso")]
pub use espresso::*;
#[cfg(feature = "eva")]
mod eva;
#[cfg(feature = "eva")]
pub use eva::*;
#[cfg(feature = "framer")]
mod framer;
#[cfg(feature = "framer")]
pub use framer::*;
#[cfg(feature = "fruit_soda")]
mod fruit_soda;
#[cfg(feature = "fruit_soda")]
pub use fruit_soda::*;
#[cfg(feature = "gigavolt")]
mod gigavolt;
#[cfg(feature = "gigavolt")]
pub use gigavolt::*;
#[cfg(feature = "github")]
mod github;
#[cfg(feature = "github")]
pub use github::*;
#[cfg(feature = "gruvbox")]
mod gruvbox;
#[cfg(feature = "gruvbox")]
pub use gruvbox::*;
#[cfg(feature = "hardcore")]
mod hardcore;
#[cfg(feature = "hardcore")]
pub use hardcore::*;
#[cfg(feature = "heetch")]
mod heetch;
#[cfg(feature = "heetch")]
pub use heetch::*;
#[cfg(feature = "helios")]
mod helios;
#[cfg(feature = "helios")]
pub use helios::*;
#[cfg(feature = "horizon")]
mod horizon;
#[cfg(feature = "horizon")]
pub use horizon::*;
#[cfg(feature = "humanoid")]
mod humanoid;
#[cfg(feature = "humanoid")]
pub use humanoid::*;
#[cfg(feature = "icy")]
mod icy;
#[cfg(feature = "icy")]
pub use icy::*;
#[cfg(feature = "kimber")]
mod kimber;
#[cfg(feature = "kimber")]
pub use kimber::*;
#[cfg(feature = "materia")]
mod materia;
#[cfg(feature = "materia")]
pub use materia::*;
#[cfg(feature = "material")]
mod material;
#[cfg(feature = "material")]
pub use material::*;
#[cfg(feature = "material_vivid")]
mod material_vivid;
#[cfg(feature = "material_vivid")]
pub use material_vivid::*;
#[cfg(feature = "mellow")]
mod mellow;
#[cfg(feature = "mellow")]
pub use mellow::*;
#[cfg(feature = "mexico_light")]
mod mexico_light;
#[cfg(feature = "mexico_light")]
pub use mexico_light::*;
#[cfg(feature = "nebula")]
mod nebula;
#[cfg(feature = "nebula")]
pub use nebula::*;
#[cfg(feature = "nord")]
mod nord;
#[cfg(feature = "nord")]
pub use nord::*;
#[cfg(feature = "nova")]
mod nova;
#[cfg(feature = "nova")]
pub use nova::*;
#[cfg(feature = "one_light")]
mod one_light;
#[cfg(feature = "one_light")]
pub use one_light::*;
#[cfg(feature = "onedark")]
mod onedark;
#[cfg(feature = "onedark")]
pub use onedark::*;
#[cfg(feature = "outrun")]
mod outrun;
#[cfg(feature = "outrun")]
pub use outrun::*;
#[cfg(feature = "papercolor")]
mod papercolor;
#[cfg(feature = "papercolor")]
pub use papercolor::*;
#[cfg(feature = "pasque")]
mod pasque;
#[cfg(feature = "pasque")]
pub use pasque::*;
#[cfg(feature = "pinky")]
mod pinky;
#[cfg(feature = "pinky")]
pub use pinky::*;
#[cfg(feature = "porple")]
mod porple;
#[cfg(feature = "porple")]
pub use porple::*;
#[cfg(feature = "purpledream")]
mod purpledream;
#[cfg(feature = "purpledream")]
pub use purpledream::*;
#[cfg(feature = "qualia")]
mod qualia;
#[cfg(feature = "qualia")]
pub use qualia::*;
#[cfg(feature = "rebecca")]
mod rebecca;
#[cfg(feature = "rebecca")]
pub use rebecca::*;
#[cfg(feature = "rose_pine")]
mod rose_pine;
#[cfg(feature = "rose_pine")]
pub use rose_pine::*;
#[cfg(feature = "sagelight")]
mod sagelight;
#[cfg(feature = "sagelight")]
pub use sagelight::*;
#[cfg(feature = "sakura")]
mod sakura;
#[cfg(feature = "sakura")]
pub use sakura::*;
#[cfg(feature = "sandcastle")]
mod sandcastle;
#[cfg(feature = "sandcastle")]
pub use sandcastle::*;
#[cfg(feature = "shades_of_purple")]
mod shades_of_purple;
#[cfg(feature = "shades_of_purple")]
pub use shades_of_purple::*;
#[cfg(feature = "silk")]
mod silk;
#[cfg(feature = "silk")]
pub use silk::*;
#[cfg(feature = "snazzy")]
mod snazzy;
#[cfg(feature = "snazzy")]
pub use snazzy::*;
#[cfg(feature = "solarflare")]
mod solarflare;
#[cfg(feature = "solarflare")]
pub use solarflare::*;
#[cfg(feature = "summercamp")]
mod summercamp;
#[cfg(feature = "summercamp")]
pub use summercamp::*;
#[cfg(feature = "summerfruit")]
mod summerfruit;
#[cfg(feature = "summerfruit")]
pub use summerfruit::*;
#[cfg(feature = "synth_midnight")]
mod synth_midnight;
#[cfg(feature = "synth_midnight")]
pub use synth_midnight::*;
#[cfg(feature = "tender")]
mod tender;
#[cfg(feature = "tender")]
pub use tender::*;
#[cfg(feature = "twilight")]
mod twilight;
#[cfg(feature = "twilight")]
pub use twilight::*;
#[cfg(feature = "unikitty")]
mod unikitty;
#[cfg(feature = "unikitty")]
pub use unikitty::*;
#[cfg(feature = "vice")]
mod vice;
#[cfg(feature = "vice")]
pub use vice::*;
#[cfg(feature = "windows")]
mod windows;
#[cfg(feature = "windows")]
pub use windows::*;
#[cfg(feature = "woodland")]
mod woodland;
#[cfg(feature = "woodland")]
pub use woodland::*;
#[cfg(feature = "xcode_dusk")]
mod xcode_dusk;
#[cfg(feature = "xcode_dusk")]
pub use xcode_dusk::*;
#[cfg(feature = "zenburn")]
mod zenburn;
#[cfg(feature = "zenburn")]
pub use zenburn::*;

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
