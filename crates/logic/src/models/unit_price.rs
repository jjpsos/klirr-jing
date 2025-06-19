use crate::prelude::*;

/// The cost of a single item, e.g. the cost of one day of consulting service.
#[derive(Clone, Copy, Display, PartialEq, Debug, Serialize, Deserialize, From, Deref)]
pub struct UnitPrice(f64);
