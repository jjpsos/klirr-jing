use crate::prelude::*;

/// The quantity of items, e.g. the number of days of consulting service.
#[derive(Clone, Copy, Display, Debug, PartialEq, Serialize, Deserialize, From, Deref)]
pub struct Quantity(f64);
