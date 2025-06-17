use crate::prelude::*;

pub trait MaybeIsExpenses {
    /// Returns true if this invoice is for expenses only.
    fn is_expenses(&self) -> bool;
}

impl MaybeIsExpenses for LineItemsPricedInSourceCurrency {
    fn is_expenses(&self) -> bool {
        self.is_expenses()
    }
}

/// Services or expenses included in this invoice to be paid by the client.
#[derive(Clone, Debug, Serialize, Deserialize, From, TryUnwrap, IsVariant)]
#[from(Vec<Item>, Item)]
pub enum LineItemsPricedInSourceCurrency {
    /// Service sold by the vendor to the client, e.g. `"App development"`
    Service(Item),
    /// Expense incurred by the vendor, travel expenses for a conference/summit/
    /// retreat
    Expenses(Vec<Item>),
}
