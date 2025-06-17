use crate::prelude::*;

/// Localization for line items in the invoice, used in the
/// table of items being billed for.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18nLineItems {
    /// EN: "Item"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    description: String,

    /// EN: "When"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    when: String,

    /// EN: "Quantity"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    quantity: String,

    /// EN: "Unit price"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    unit_price: String,

    /// EN: "Total cost"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    total_cost: String,

    /// EN: "Grand Total:"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    grand_total: String,
}

impl L18nLineItems {
    pub fn english() -> Self {
        Self::builder()
            .description("Item")
            .when("When")
            .quantity("Quantity")
            .unit_price("Unit price")
            .total_cost("Total cost")
            .grand_total("Grand Total:")
            .build()
    }
}
