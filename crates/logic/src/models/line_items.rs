use std::collections::HashMap;

use derive_more::TryUnwrap;

use crate::prelude::*;

/// Services or expenses included in this invoice to be paid by the client.
#[derive(Clone, Debug, Serialize, Deserialize, From, TryUnwrap)]
#[from(Vec<Item>, Item)]
pub enum LineItemsWithoutCost {
    /// Service sold by the vendor to the client, e.g. `"App development"`
    Service(ItemWithoutCost),
    /// Expense incurred by the vendor, travel expenses for a conference/summit/
    /// retreat
    Expenses(Vec<ItemWithoutCost>),
}

#[derive(Clone, Debug, Serialize, TypedBuilder, Getters)]
pub struct ExchangeRates {
    /// MUST Match the currency of the invoice, e.g. `"EUR"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    target_currency: Currency,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    rates: HashMap<Currency, UnitPrice>,
}
impl ExchangeRates {
    pub fn convert(&self, unit_price: UnitPrice, currency: Currency) -> Result<UnitPrice> {
        if self.target_currency == currency {
            return Ok(unit_price);
        }
        let rate = self
            .rates
            .get(&currency)
            .ok_or(Error::FoundNoExchangeRate {
                target: self.target_currency,
                base: currency,
            })?;
        Ok(UnitPrice::from(*unit_price * **rate))
    }
}

impl TryFrom<(LineItemsWithoutCost, ExchangeRates)> for LineItemsFlat {
    type Error = crate::prelude::Error;
    fn try_from(
        (line_items, exchange_rates): (LineItemsWithoutCost, ExchangeRates),
    ) -> Result<Self> {
        match line_items {
            LineItemsWithoutCost::Service(service) => {
                let service =
                    service.converting_currency_and_calculating_total_cost(&exchange_rates)?;
                let flat = LineItemsFlat::builder()
                    .service(service)
                    .expenses(vec![])
                    .build();
                Ok(flat)
            }
            LineItemsWithoutCost::Expenses(expenses) => {
                let expenses = expenses
                    .into_iter()
                    .map(|expense| {
                        expense.converting_currency_and_calculating_total_cost(&exchange_rates)
                    })
                    .collect::<Result<Vec<_>>>()?;
                let flat = LineItemsFlat::builder().expenses(expenses).build();
                Ok(flat)
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Getters, TypedBuilder)]
pub struct LineItemsFlat {
    /// Service sold by the vendor to the client, e.g. `"App development"`,
    /// might be `None` if no service is provided
    #[builder(setter(into, strip_option), default = None)]
    #[getset(get = "pub")]
    service: Option<ItemWithCost>,

    /// Expense incurred by the vendor, travel expenses for a conference/summit/
    /// retreat,
    /// might be empty if no expenses are incurred
    #[getset(get = "pub")]
    expenses: Vec<ItemWithCost>,
}
