use crate::prelude::*;

/// A unique number for the invoice, e.g. `90`
#[derive(Clone, Debug, Display, Serialize, Deserialize, PartialEq, Eq, From, Deref)]
#[serde(transparent)]
pub struct InvoiceNumber(u16);

impl InvoiceNumber {
    pub fn sample() -> Self {
        Self::from(9876)
    }
}
