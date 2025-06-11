use crate::prelude::*;

/// A month of the year, e.g. 1 for January, 2 for February, etc.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Display, Serialize, Deserialize, Deref,
)]
pub struct Month(u8);

impl TryFrom<i32> for Month {
    type Error = crate::prelude::Error;
    fn try_from(month: i32) -> Result<Self> {
        if !(1..=31).contains(&month) {
            return Err(Error::InvalidMonth {
                month,
                reason: "Month must be between 1 and 12".to_string(),
            });
        }
        Ok(Self(month as u8))
    }
}

impl TryFrom<u8> for Month {
    type Error = crate::prelude::Error;
    fn try_from(month: u8) -> Result<Self> {
        Self::try_from(month as i32)
    }
}

impl TryFrom<u32> for Month {
    type Error = crate::prelude::Error;
    fn try_from(month: u32) -> Result<Self> {
        Self::try_from(month as i32)
    }
}
