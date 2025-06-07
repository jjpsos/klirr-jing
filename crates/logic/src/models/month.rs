use crate::prelude::*;

/// A month of the year, e.g. 1 for January, 2 for February, etc.
#[derive(Clone, Copy, Debug, Display, Serialize, Deserialize, Deref)]
pub struct Month(u8);

impl From<i32> for Month {
    fn from(month: i32) -> Self {
        assert!(month >= 1 && month <= 12, "Month must be between 1 and 12");
        Self(month as u8)
    }
}
