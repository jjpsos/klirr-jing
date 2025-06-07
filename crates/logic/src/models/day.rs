use crate::prelude::*;

/// The day of the month, e.g. 1 for the first day, 31 for the last day of a month.
#[derive(Clone, Copy, Debug, Display, Serialize, Deserialize, From, Deref)]
pub struct Day(u8);

impl From<i32> for Day {
    fn from(day: i32) -> Self {
        assert!(day >= 1 && day <= 31, "Month must be between 1 and 31");
        Self(day as u8)
    }
}
