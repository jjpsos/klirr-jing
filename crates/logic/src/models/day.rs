use derive_more::FromStr;

use crate::prelude::*;

/// The day of the month, e.g. 1 for the first day, 31 for the last day of a month.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, Serialize, Deserialize, Deref, FromStr)]
pub struct Day(u8);

impl HasSample for Day {
    fn sample() -> Self {
        Self(1)
    }
}

impl TryFrom<i32> for Day {
    type Error = crate::prelude::Error;
    fn try_from(day: i32) -> Result<Self> {
        if !(1..=31).contains(&day) {
            return Err(Error::InvalidDay {
                day,
                reason: "Day must be between 1 and 31".to_string(),
            });
        }
        Ok(Self(day as u8))
    }
}

impl TryFrom<u8> for Day {
    type Error = crate::prelude::Error;
    fn try_from(day: u8) -> Result<Self> {
        Self::try_from(day as i32)
    }
}

impl TryFrom<u32> for Day {
    type Error = crate::prelude::Error;
    fn try_from(day: u32) -> Result<Self> {
        Self::try_from(day as i32)
    }
}
