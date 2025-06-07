use crate::prelude::*;

/// The day of the month, e.g. 1 for the first day, 31 for the last day of a month.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Display, Serialize, Deserialize, From, Deref)]
pub struct Day(u8);

impl From<i32> for Day {
    fn from(day: i32) -> Self {
        assert!((1..=31).contains(&day), "Month must be between 1 and 31");
        Self(day as u8)
    }
}

impl From<u32> for Day {
    fn from(day: u32) -> Self {
        Self::from(day as i32)
    }
}
