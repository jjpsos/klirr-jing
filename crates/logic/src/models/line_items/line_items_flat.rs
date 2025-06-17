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
