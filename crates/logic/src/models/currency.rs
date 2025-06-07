use std::fmt;

use serde::{
    Deserialize, Deserializer, Serializer,
    de::{self, Visitor},
};

use crate::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Currency {
    /// 🇺🇸 United States Dollar (USD), used in the United States
    USD,
    /// 🇪🇺 Euro (EUR), used in eurozone countries such as Germany, France, Italy, etc.
    EUR,
    /// 🇬🇧 British Pound Sterling (GBP), used in the United Kingdom
    GBP,
    /// 🇯🇵 Japanese Yen (JPY), used in Japan
    JPY,
    /// 🇨🇦 Canadian Dollar (CAD), used in Canada
    CAD,
    /// 🇦🇺 Australian Dollar (AUD), used in Australia
    AUD,
    /// 🇨🇭 Swiss Franc (CHF), used in Switzerland and Liechtenstein
    CHF,
    /// 🇸🇪 Swedish Krona (SEK), used in Sweden
    SEK,
    /// 🇳🇴 Norwegian Krone (NOK), used in Norway
    NOK,
    /// 🇩🇰 Danish Krone (DKK), used in Denmark
    DKK,
    /// 🇨🇳 Chinese Yuan (CNY), used in China
    CNY,
    /// 🇮🇳 Indian Rupee (INR), used in India
    INR,
    /// 🇧🇷 Brazilian Real (BRL), used in Brazil
    BRL,
    /// 🇷🇺 Russian Ruble (RUB), used in Russia
    RUB,
    /// 🇿🇦 South African Rand (ZAR), used in South Africa
    ZAR,
    /// 🇲🇽 Mexican Peso (MXN), used in Mexico
    MXN,
    /// 🇳🇿 New Zealand Dollar (NZD), used in New Zealand
    NZD,
    /// 🇸🇬 Singapore Dollar (SGD), used in Singapore
    SGD,
    /// 🇭🇰 Hong Kong Dollar (HKD), used in Hong Kong
    HKD,
}

// Display implementation to return ISO code
impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let code = match self {
            Currency::USD => "USD",
            Currency::EUR => "EUR",
            Currency::GBP => "GBP",
            Currency::JPY => "JPY",
            Currency::CAD => "CAD",
            Currency::AUD => "AUD",
            Currency::CHF => "CHF",
            Currency::SEK => "SEK",
            Currency::NOK => "NOK",
            Currency::DKK => "DKK",
            Currency::CNY => "CNY",
            Currency::INR => "INR",
            Currency::BRL => "BRL",
            Currency::RUB => "RUB",
            Currency::ZAR => "ZAR",
            Currency::MXN => "MXN",
            Currency::NZD => "NZD",
            Currency::SGD => "SGD",
            Currency::HKD => "HKD",
        };
        write!(f, "{}", code)
    }
}

// Serde serialization using ISO code
impl Serialize for Currency {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// Serde deserialization using ISO code
impl<'de> Deserialize<'de> for Currency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct CurrencyVisitor;

        impl Visitor<'_> for CurrencyVisitor {
            type Value = Currency;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("a valid ISO 4217 currency code string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Currency, E>
            where
                E: de::Error,
            {
                match value {
                    "USD" => Ok(Currency::USD),
                    "EUR" => Ok(Currency::EUR),
                    "GBP" => Ok(Currency::GBP),
                    "JPY" => Ok(Currency::JPY),
                    "CAD" => Ok(Currency::CAD),
                    "AUD" => Ok(Currency::AUD),
                    "CHF" => Ok(Currency::CHF),
                    "SEK" => Ok(Currency::SEK),
                    "NOK" => Ok(Currency::NOK),
                    "DKK" => Ok(Currency::DKK),
                    "CNY" => Ok(Currency::CNY),
                    "INR" => Ok(Currency::INR),
                    "BRL" => Ok(Currency::BRL),
                    "RUB" => Ok(Currency::RUB),
                    "ZAR" => Ok(Currency::ZAR),
                    "MXN" => Ok(Currency::MXN),
                    "NZD" => Ok(Currency::NZD),
                    "SGD" => Ok(Currency::SGD),
                    "HKD" => Ok(Currency::HKD),
                    _ => Err(E::custom(format!("unknown currency code: {}", value))),
                }
            }
        }

        deserializer.deserialize_str(CurrencyVisitor)
    }
}
