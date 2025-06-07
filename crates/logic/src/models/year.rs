use crate::prelude::*;

/// Years since birth of Jesus christ, e.g. 2025
#[derive(Clone, Copy, Debug, Display, Serialize, Deserialize, From, Deref)]
pub struct Year(u16);

impl From<i32> for Year {
    fn from(year: i32) -> Self {
        Self(year as u16)
    }
}
