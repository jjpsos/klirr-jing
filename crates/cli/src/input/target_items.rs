use derive_more::IsVariant;

use crate::prelude::*;

#[derive(Subcommand, Debug, Clone, IsVariant, PartialEq)]
pub enum TargetItems {
    /// Mark yourself out of office for N days
    Ooo {
        /// Number of days
        days: u8,
    },
    /// Expenses mode, specify expenses in `input/data/expenses.json` for the
    /// target month.
    Expenses,
}
impl Default for TargetItems {
    fn default() -> Self {
        TargetItems::Ooo { days: 0 }
    }
}
