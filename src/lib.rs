pub mod palettes;

use enum_dispatch::enum_dispatch;
use palettes::*;

#[enum_dispatch(Base16Palette)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Palette {
    DefaultPalette,
    #[cfg(feature = "apprentice")]
    ApprenticePalette,
    #[cfg(feature = "atelier")]
    AtelierPalette,
    #[cfg(feature = "atlas")]
    AtlasPalette,
    #[cfg(feature = "black_metal")]
    BlackMetalPalette,
    #[cfg(feature = "brogrammer")]
    BrogrammerPalette,
    #[cfg(feature = "brush_trees")]
    BrushTreesPalette,
    #[cfg(feature = "circus")]
    CircusPalette,
    #[cfg(feature = "classic")]
    ClassicPalette,
    #[cfg(feature = "codeschool")]
    CodeschoolPalette,
    #[cfg(feature = "colors")]
    ColorsPalette,
    #[cfg(feature = "cupertino")]
    CupertinoPalette,
    #[cfg(feature = "danqing")]
    DanqingPalette,
    #[cfg(feature = "darcula")]
    DarculaPalette,
    #[cfg(feature = "darkviolet")]
    DarkvioletPalette,
    #[cfg(feature = "dracula")]
    DraculaPalette,
    #[cfg(feature = "equilibrium")]
    EquilibriumPalette,
    #[cfg(feature = "espresso")]
    EspressoPalette,
    #[cfg(feature = "eva")]
    EvaPalette,
    #[cfg(feature = "framer")]
    FramerPalette,
    #[cfg(feature = "fruit_soda")]
    FruitSodaPalette,
    #[cfg(feature = "gigavolt")]
    GigavoltPalette,
    #[cfg(feature = "github")]
    GithubPalette,
    #[cfg(feature = "gruvbox")]
    GruvboxPalette,
    #[cfg(feature = "hardcore")]
    HardcorePalette,
    #[cfg(feature = "heetch")]
    HeetchPalette,
    #[cfg(feature = "helios")]
    HeliosPalette,
    #[cfg(feature = "horizon")]
    HorizonPalette,
    #[cfg(feature = "humanoid")]
    HumanoidPalette,
    #[cfg(feature = "icy")]
    IcyPalette,
    #[cfg(feature = "kimber")]
    KimberPalette,
    #[cfg(feature = "materia")]
    MateriaPalette,
    #[cfg(feature = "material")]
    MaterialPalette,
    #[cfg(feature = "material_vivid")]
    MaterialVividPalette,
    #[cfg(feature = "mellow")]
    MellowPalette,
    #[cfg(feature = "mexico_light")]
    MexicoLightPalette,
    #[cfg(feature = "nebula")]
    NebulaPalette,
    #[cfg(feature = "nord")]
    NordPalette,
    #[cfg(feature = "nova")]
    NovaPalette,
    #[cfg(feature = "one_light")]
    OneLightPalette,
    #[cfg(feature = "onedark")]
    OnedarkPalette,
    #[cfg(feature = "outrun")]
    OutrunPalette,
    #[cfg(feature = "papercolor")]
    PapercolorPalette,
    #[cfg(feature = "pasque")]
    PasquePalette,
    #[cfg(feature = "pinky")]
    PinkyPalette,
    #[cfg(feature = "porple")]
    PorplePalette,
    #[cfg(feature = "purpledream")]
    PurpleDreamPalette,
    #[cfg(feature = "qualia")]
    QualiaPalette,
    #[cfg(feature = "rebecca")]
    RebeccaPalette,
    #[cfg(feature = "rose_pine")]
    RosePinePalette,
    #[cfg(feature = "sagelight")]
    SagelightPalette,
    #[cfg(feature = "sakura")]
    SakuraPalette,
    #[cfg(feature = "sandcastle")]
    SandcastlePalette,
    #[cfg(feature = "shades_of_purple")]
    ShadesOfPurplePalette,
    #[cfg(feature = "silk")]
    SilkPalette,
    #[cfg(feature = "snazzy")]
    SnazzyPalette,
    #[cfg(feature = "solarflare")]
    SolarflarePalette,
    #[cfg(feature = "summercamp")]
    SummercampPalette,
    #[cfg(feature = "summerfruit")]
    SummerfruitPalette,
    #[cfg(feature = "synth_midnight")]
    SynthMidnightPalette,
    #[cfg(feature = "tender")]
    TenderPalette,
    #[cfg(feature = "twilight")]
    TwilightPalette,
    #[cfg(feature = "unikitty")]
    UnikittyPalette,
    #[cfg(feature = "vice")]
    VicePalette,
    #[cfg(feature = "windows")]
    WindowsPalette,
    #[cfg(feature = "woodland")]
    WoodlandPalette,
    #[cfg(feature = "xcode_dusk")]
    XcodeDuskPalette,
    #[cfg(feature = "zenburn")]
    ZenburnPalette,
}

impl Default for Palette {
    fn default() -> Self {
        Self::DefaultPalette(DefaultPalette::default())
    }
}

/// The universal representation of a Base16 color palette.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base16Color {
    Shade(Base16Shade),
    Accent(Base16Accent),
}

/// Every Base16 color palette contains 8 "shades". These are split between 4 "dark" and 4 "light"
/// shades.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base16Shade {
    Dark(Shade),
    Light(Shade),
}

/// Base16 shades are split into two 4-value gradients.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shade {
    Darkest,
    Darker,
    Lighter,
    Lightest,
}

/// Every Base16 color palette contains 8 "accents".
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Base16Accent {
    Accent00,
    Accent01,
    Accent02,
    Accent03,
    Accent04,
    Accent05,
    Accent06,
    Accent07,
}

impl Base16Color {
    pub const fn index(self) -> u8 {
        match self {
            Base16Color::Shade(Base16Shade::Dark(Shade::Darkest)) => 0,
            Base16Color::Shade(Base16Shade::Dark(Shade::Darker)) => 1,
            Base16Color::Shade(Base16Shade::Dark(Shade::Lighter)) => 2,
            Base16Color::Shade(Base16Shade::Dark(Shade::Lightest)) => 3,
            Base16Color::Shade(Base16Shade::Light(Shade::Darkest)) => 4,
            Base16Color::Shade(Base16Shade::Light(Shade::Darker)) => 5,
            Base16Color::Shade(Base16Shade::Light(Shade::Lighter)) => 6,
            Base16Color::Shade(Base16Shade::Light(Shade::Lightest)) => 7,
            Base16Color::Accent(Base16Accent::Accent00) => 8,
            Base16Color::Accent(Base16Accent::Accent01) => 9,
            Base16Color::Accent(Base16Accent::Accent02) => 10,
            Base16Color::Accent(Base16Accent::Accent03) => 11,
            Base16Color::Accent(Base16Accent::Accent04) => 12,
            Base16Color::Accent(Base16Accent::Accent05) => 13,
            Base16Color::Accent(Base16Accent::Accent06) => 14,
            Base16Color::Accent(Base16Accent::Accent07) => 15,
        }
    }

    pub const fn from_index(i: u8) -> Self {
        match i {
            0 => Base16Color::Shade(Base16Shade::Dark(Shade::Darkest)),
            1 => Base16Color::Shade(Base16Shade::Dark(Shade::Darker)),
            2 => Base16Color::Shade(Base16Shade::Dark(Shade::Lighter)),
            3 => Base16Color::Shade(Base16Shade::Dark(Shade::Lightest)),
            4 => Base16Color::Shade(Base16Shade::Light(Shade::Darkest)),
            5 => Base16Color::Shade(Base16Shade::Light(Shade::Darker)),
            6 => Base16Color::Shade(Base16Shade::Light(Shade::Lighter)),
            7 => Base16Color::Shade(Base16Shade::Light(Shade::Lightest)),
            8 => Base16Color::Accent(Base16Accent::Accent00),
            9 => Base16Color::Accent(Base16Accent::Accent01),
            10 => Base16Color::Accent(Base16Accent::Accent02),
            11 => Base16Color::Accent(Base16Accent::Accent03),
            12 => Base16Color::Accent(Base16Accent::Accent04),
            13 => Base16Color::Accent(Base16Accent::Accent05),
            14 => Base16Color::Accent(Base16Accent::Accent06),
            15 => Base16Color::Accent(Base16Accent::Accent07),
            // Maybe use a default value?
            _ => panic!("Unknown color code!"),
        }
    }
}

#[enum_dispatch]
pub trait Base16Palette {
    fn to_rgb(&self, color: Base16Color) -> (u8, u8, u8);
    fn to_hex_str(&self, color: Base16Color) -> &'static str;
}

impl Base16Color {
    pub const fn default_fg() -> Self {
        Self::light_3()
    }

    pub const fn default_bg() -> Self {
        Self::dark_2()
    }

    pub const fn dark_1() -> Self {
        Self::Shade(Base16Shade::Dark(Shade::Darkest))
    }

    pub const fn dark_2() -> Self {
        Self::Shade(Base16Shade::Dark(Shade::Darker))
    }

    pub const fn dark_3() -> Self {
        Self::Shade(Base16Shade::Dark(Shade::Lighter))
    }

    pub const fn dark_4() -> Self {
        Self::Shade(Base16Shade::Dark(Shade::Lightest))
    }

    pub const fn light_1() -> Self {
        Self::Shade(Base16Shade::Light(Shade::Darkest))
    }

    pub const fn light_2() -> Self {
        Self::Shade(Base16Shade::Light(Shade::Darker))
    }

    pub const fn light_3() -> Self {
        Self::Shade(Base16Shade::Light(Shade::Lighter))
    }

    pub const fn light_4() -> Self {
        Self::Shade(Base16Shade::Light(Shade::Lightest))
    }
}
