use crate::prelude::*;

/// The items being invoiced this month, either services or expenses.
#[derive(Clone, Debug, Display, Serialize, Deserialize, IsVariant)]
pub enum InvoicedItems {
    #[display("Service {{ days_off: {} }} ", days_off.map(|d| *d).unwrap_or(0))]
    Service { days_off: Option<Day> },
    #[display("Expenses")]
    Expenses,
}
impl MaybeIsExpenses for InvoicedItems {
    fn is_expenses(&self) -> bool {
        self.is_expenses()
    }
}
