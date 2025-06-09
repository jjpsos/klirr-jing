use crate::prelude::*;

/// A unique number for the invoice, e.g. `90`
#[derive(Clone, Debug, Serialize, Deserialize, From, Deref)]
#[serde(transparent)]
pub struct InvoiceNumber(u16);
