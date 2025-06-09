use serde_with::DeserializeFromStr;

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, DeserializeFromStr, Deref)]
#[serde(transparent)]
pub struct HexColor(String);

impl FromStr for HexColor {
    type Err = crate::prelude::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with('#') && (s.len() == 7 || s.len() == 9) {
            Ok(HexColor(s.to_string()))
        } else {
            Err(Error::InvalidHexColor {
                invalid_string: s.to_string(),
            })
        }
    }
}
