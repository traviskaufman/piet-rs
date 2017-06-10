use std::fmt;

use ansi_term::Colour::RGB;

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum Hue {
    NoHue = 0,
    Red = 1,
    Yellow = 2,
    Green = 3,
    Cyan = 4,
    Blue = 5,
    Magenta = 6,
}

#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq)]
pub enum Lightness {
    NoLightness = 0,
    Light = 1,
    Normal = 2,
    Dark = 3,
}

#[derive(Debug)]
pub struct Color {
    pub mnemonic: &'static str,
    pub rgb_color: (u8, u8, u8),
    pub hue: Hue,
    pub lightness: Lightness,
}

impl Color {
    pub fn from_px(px: &(u8, u8, u8)) -> Result<Color, String> {
        match *px {
            (255, 192, 192) => {
                Ok(Color {
                    mnemonic: "LightRed",
                    rgb_color: *px,
                    hue: Hue::Red,
                    lightness: Lightness::Light,
                })
            }
            (255, 0, 0) => {
                Ok(Color {
                    mnemonic: "Red",
                    rgb_color: *px,
                    hue: Hue::Red,
                    lightness: Lightness::Normal,
                })
            }
            (192, 0, 0) => {
                Ok(Color {
                    mnemonic: "DarkRed",
                    rgb_color: *px,
                    hue: Hue::Red,
                    lightness: Lightness::Dark,
                })
            }
            (255, 255, 192) => {
                Ok(Color {
                    mnemonic: "LightYellow",
                    rgb_color: *px,
                    hue: Hue::Yellow,
                    lightness: Lightness::Light,
                })
            }
            (255, 255, 0) => {
                Ok(Color {
                    mnemonic: "Yellow",
                    rgb_color: *px,
                    hue: Hue::Yellow,
                    lightness: Lightness::Normal,
                })
            }
            (192, 192, 0) => {
                Ok(Color {
                    mnemonic: "DarkYellow",
                    rgb_color: *px,
                    hue: Hue::Yellow,
                    lightness: Lightness::Dark,
                })
            }
            (192, 255, 192) => {
                Ok(Color {
                    mnemonic: "LightGreen",
                    rgb_color: *px,
                    hue: Hue::Green,
                    lightness: Lightness::Light,
                })
            }
            (0, 255, 0) => {
                Ok(Color {
                    mnemonic: "Green",
                    rgb_color: *px,
                    hue: Hue::Green,
                    lightness: Lightness::Normal,
                })
            }
            (0, 192, 0) => {
                Ok(Color {
                    mnemonic: "DarkGreen",
                    rgb_color: *px,
                    hue: Hue::Green,
                    lightness: Lightness::Dark,
                })
            }
            (192, 255, 255) => {
                Ok(Color {
                    mnemonic: "LightCyan",
                    rgb_color: *px,
                    hue: Hue::Cyan,
                    lightness: Lightness::Light,
                })
            }
            (0, 255, 255) => {
                Ok(Color {
                    mnemonic: "Cyan",
                    rgb_color: *px,
                    hue: Hue::Cyan,
                    lightness: Lightness::Normal,
                })
            }
            (0, 192, 192) => {
                Ok(Color {
                    mnemonic: "DarkCyan",
                    rgb_color: *px,
                    hue: Hue::Cyan,
                    lightness: Lightness::Dark,
                })
            }
            (192, 192, 255) => {
                Ok(Color {
                    mnemonic: "LightBlue",
                    rgb_color: *px,
                    hue: Hue::Blue,
                    lightness: Lightness::Light,
                })
            }
            (0, 0, 255) => {
                Ok(Color {
                    mnemonic: "Blue",
                    rgb_color: *px,
                    hue: Hue::Blue,
                    lightness: Lightness::Normal,
                })
            }
            (0, 0, 192) => {
                Ok(Color {
                    mnemonic: "DarkBlue",
                    rgb_color: *px,
                    hue: Hue::Blue,
                    lightness: Lightness::Dark,
                })
            }
            (255, 192, 255) => {
                Ok(Color {
                    mnemonic: "LightMagenta",
                    rgb_color: *px,
                    hue: Hue::Magenta,
                    lightness: Lightness::Light,
                })
            }
            (255, 0, 255) => {
                Ok(Color {
                    mnemonic: "Magenta",
                    rgb_color: *px,
                    hue: Hue::Magenta,
                    lightness: Lightness::Normal,
                })
            }
            (192, 0, 192) => {
                Ok(Color {
                    mnemonic: "DarkMagenta",
                    rgb_color: *px,
                    hue: Hue::Magenta,
                    lightness: Lightness::Dark,
                })
            }
            _ => Err(String::from(format!("Invalid color specified: {:?}", px))),
        }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (r, g, b) = self.rgb_color;
        write!(f, "{}", RGB(r, g, b).paint(self.mnemonic))
    }
}
