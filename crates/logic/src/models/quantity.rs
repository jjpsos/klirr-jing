use crate::prelude::*;

/// The quantity of items, e.g. the number of days of consulting service.
#[derive(Clone, Copy, Display, Debug, PartialEq, Serialize, Deserialize, From, Deref)]
pub struct Quantity(f64);

impl HasSample for Quantity {
    fn sample() -> Self {
        Self::from(1.0)
    }
}
