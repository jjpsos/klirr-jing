use crate::prelude::*;

/// A unique identifier for the invoice, e.g. `"90"`, `"INV-2025-001"`.
#[derive(Clone, Debug, Serialize, Deserialize, From, Deref)]
#[from(String, &'static str)]
#[serde(transparent)]
pub struct InvoiceIdentifier(String);
