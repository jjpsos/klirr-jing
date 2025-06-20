use derive_more::Constructor;

use crate::prelude::*;

/// Years since birth of Jesus christ, e.g. 2025
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    Display,
    Serialize,
    Deserialize,
    From,
    Deref,
    Constructor,
)]
pub struct Year(u16);

impl From<i32> for Year {
    /// Converts an `i32` year to a `Year`.
    /// # Examples
    /// ```
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    /// let year: Year = 2025.into();
    /// assert_eq!(*year, 2025);
    /// ```
    fn from(year: i32) -> Self {
        Self(year as u16)
    }
}
