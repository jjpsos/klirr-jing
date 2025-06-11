use std::collections::HashMap;

use derive_more::{IsVariant, TryUnwrap};

use crate::prelude::*;

pub trait MaybeIsExpenses {
    /// Returns true if this invoice is for expenses only.
    fn is_expenses(&self) -> bool;
}

impl MaybeIsExpenses for LineItemsWithoutCost {
    fn is_expenses(&self) -> bool {
        self.is_expenses()
    }
}

/// Services or expenses included in this invoice to be paid by the client.
#[derive(Clone, Debug, Serialize, Deserialize, From, TryUnwrap, IsVariant)]
#[from(Vec<Item>, Item)]
pub enum LineItemsWithoutCost {
    /// Service sold by the vendor to the client, e.g. `"App development"`
    Service(Item),
    /// Expense incurred by the vendor, travel expenses for a conference/summit/
    /// retreat
    Expenses(Vec<Item>),
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
                    .items(vec![service])
                    .is_expenses(false)
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
                let flat = LineItemsFlat::builder()
                    .items(expenses)
                    .is_expenses(true)
                    .build();
                Ok(flat)
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Getters, TypedBuilder)]
pub struct LineItemsFlat {
    /// True if this invoice is for expenses only.
    #[getset(get = "pub")]
    is_expenses: bool,

    /// Either a single item (Serivec) or one or more expenses
    #[getset(get = "pub")]
    items: Vec<ItemConvertedIntoTargetCurrency>,
}

impl MaybeIsExpenses for LineItemsFlat {
    fn is_expenses(&self) -> bool {
        self.is_expenses
    }
}
