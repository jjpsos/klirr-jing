use crate::prelude::*;

/// A date relevant for the invoice, e.g. invoice date, due date or a transaction
/// date for an expense.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct Date {
    /// e.g. 2025
    #[builder(setter(into))]
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    month: Month,

    /// e.g. 31 for the last day of May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    day: Day,
}
