use serde_with::DeserializeFromStr;

use crate::prelude::*;

#[derive(Clone, Debug, SerializeDisplay, DeserializeFromStr, derive_more::Display)]
#[display("#{:02x}{:02x}{:02x}", red, green, blue)]
pub struct HexColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl Default for HexColor {
    fn default() -> Self {
        Self::from_str("#3ed6c0").expect("Failed to create default HexColor")
    }
}

impl HexColor {
    pub fn sample() -> Self {
        Self::from_str("#e6007a").expect("Failed to create sample HexColor")
    }
}

impl FromStr for HexColor {
    type Err = crate::prelude::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') && (s.len() == 7 || s.len() == 9) {
            let s = &s[1..];
            let parse_u8 = |start: usize, end: usize| {
                u8::from_str_radix(&s[start..end], 16).map_err(|_| Error::InvalidHexColor {
                    invalid_string: s.to_string(),
                })
            };
            let red = parse_u8(0, 2)?;
            let green = parse_u8(2, 4)?;
            let blue = parse_u8(4, 6)?;
            Ok(Self { red, green, blue })
        } else {
            Err(Error::InvalidHexColor {
                invalid_string: s.to_string(),
            })
        }
    }
}
