use crate::prelude::*;

/// Services or expenses included in this invoice to be paid by the client.
#[derive(Clone, Debug, Serialize, Deserialize, From)]
#[from(Vec<Item>, Item)]
pub enum LineItems {
    /// Service sold by the vendor to the client, e.g. `"App development"`
    Service(Item),
    /// Expense incurred by the vendor, travel expenses for a conference/summit/
    /// retreat
    Expenses(Vec<Item>),
}
