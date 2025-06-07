use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct ItemWithoutCost {
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
    unit_price: UnitPrice,
    /// The quantity of the expense, e.g. `2.0` for two items
    #[builder(setter(into))]
    #[getset(get = "pub")]
    quantity: Quantity,
    /// The currency of the expense, e.g. `"EUR"`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    currency: Currency,
}

/// An item with a total cost, calculated as `unit_price * quantity`.
#[derive(Clone, Debug, Serialize, Deserialize, Deref, From, Getters, TypedBuilder)]
pub struct ItemWithCost {
    #[deref]
    without_cost: ItemWithoutCost,

    /// The total cost of the item, calculated as `unit_price * quantity`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    total_cost: Cost,
}

#[derive(Clone, Debug, Serialize, Deserialize, From, Deref)]
pub struct Quantity(f64);

#[derive(Clone, Debug, Default, Serialize, Deserialize, From, Deref)]
pub struct Cost(f64);

#[derive(Clone, Debug, Serialize, Deserialize, From, Deref)]
pub struct UnitPrice(f64);

impl ItemWithoutCost {
    pub fn converting_currency_and_calculating_total_cost(
        self,
        exchange_rates: &ExchangeRates,
    ) -> Result<ItemWithCost> {
        let converted_rates = self.with_exchange_rates(exchange_rates)?;
        Ok(converted_rates.with_total_cost())
    }

    fn with_total_cost(self) -> ItemWithCost {
        let cost = Cost::from(**self.quantity() * **self.unit_price());
        ItemWithCost::builder()
            .without_cost(self.clone())
            .total_cost(cost)
            .build()
    }

    fn with_exchange_rates(self, exchange_rates: &ExchangeRates) -> Result<Self> {
        let converted_unit_price = exchange_rates.convert(self.unit_price, self.currency)?;
        Ok(Self::builder()
            .transaction_date(self.transaction_date)
            .name(self.name)
            .unit_price(converted_unit_price)
            .quantity(self.quantity)
            .currency(*exchange_rates.target_currency())
            .build())
    }
}
