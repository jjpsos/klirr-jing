use crate::prelude::*;

/// An item with a total cost, calculated as `unit_price * quantity`.
#[derive(Clone, Debug, Serialize, Deserialize, Deref, From, Getters, TypedBuilder)]
pub struct ItemConvertedIntoTargetCurrency {
    /// An item in the currency it was paid in.
    #[deref]
    #[serde(flatten)]
    in_source_currency: Item,

    /// The total cost of the item, calculated as `unit_price * quantity`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    total_cost: Cost,
}
