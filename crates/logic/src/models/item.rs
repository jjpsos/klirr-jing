use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct Item {
    /// The date of the expense, e.g. `2025-05-31`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    transaction_date: Date,
    /// The short name of the expense, e.g. `"Coffee"`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    name: String,
    /// The cost per item
    #[builder(setter(into))]
    #[getset(get = "pub")]
    unit_price: f64,
    /// The quantity of the expense, e.g. `2.0` for two items
    #[builder(setter(into))]
    #[getset(get = "pub")]
    quantity: f64,
    /// The currency of the expense, e.g. `"EUR"`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    currency: Currency,
}
