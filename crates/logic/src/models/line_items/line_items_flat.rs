use crate::prelude::*;

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

impl TryFrom<(LineItemsPricedInSourceCurrency, ExchangeRates)> for LineItemsFlat {
    type Error = crate::prelude::Error;

    /// Converts the line items priced in source currency into a flat list of items
    /// priced in the target currency, using the provided exchange rates.
    fn try_from(
        (line_items, exchange_rates): (LineItemsPricedInSourceCurrency, ExchangeRates),
    ) -> Result<Self> {
        match line_items {
            LineItemsPricedInSourceCurrency::Service(service) => {
                let service =
                    service.converting_currency_and_calculating_total_cost(&exchange_rates)?;
                let flat = LineItemsFlat::builder()
                    .items(vec![service])
                    .is_expenses(false)
                    .build();
                Ok(flat)
            }
            LineItemsPricedInSourceCurrency::Expenses(expenses) => {
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

impl HasSample for LineItemsFlat {
    fn sample() -> Self {
        Self::builder()
            .is_expenses(false)
            .items(vec![ItemConvertedIntoTargetCurrency::sample()])
            .build()
    }
}
