use core::fmt;

use strum::{EnumIter, IntoEnumIterator};

use crate::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq, Hash, DeserializeFromStr, SerializeDisplay, EnumIter)]
pub enum Language {
    /// 🇬🇧 English
    EN,
    /// 🇸🇪 Swedish
    SV,
}

impl Language {
    pub fn all() -> impl Iterator<Item = Self> {
        Self::iter()
    }
}
impl fmt::Debug for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let debug_str = match self {
            Language::EN => "EN",
            Language::SV => "SV",
        };
        write!(f, "{}", debug_str)
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Language::EN => "english",
            Language::SV => "swedish",
        };
        write!(f, "{}", name)
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "en" | "english" => Ok(Language::EN),
            "sv" | "swedish" => Ok(Language::SV),
            _ => Err(format!("unknown language code: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_debug_snapshot;
    use test_log::test;

    #[test]
    fn test_language_debug() {
        assert_debug_snapshot!(Language::EN, @"EN");
        assert_debug_snapshot!(Language::SV, @"SV");
    }

    #[test]
    fn test_language_display() {
        assert_eq!(Language::EN.to_string(), "english");
        assert_eq!(Language::SV.to_string(), "swedish");
    }

    #[test]
    fn test_language_from_str() {
        assert_eq!("en".parse::<Language>().unwrap(), Language::EN);
        assert_eq!("sv".parse::<Language>().unwrap(), Language::SV);
        assert!("unknown".parse::<Language>().is_err());
    }
}
