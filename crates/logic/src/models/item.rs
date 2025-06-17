use std::str::FromStr;

use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct ConsultingService {
    /// Description of the consulting service, e.g. `"App development"`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    name: String,
    /// The cost per item
    #[builder(setter(into))]
    #[getset(get = "pub")]
    unit_price: UnitPrice,
}
impl ConsultingService {
    pub fn sample() -> Self {
        Self::builder()
            .name("App development".to_string())
            .unit_price(UnitPrice::from(350.0))
            .build()
    }
}

#[derive(Clone, Debug, Display, PartialEq, Serialize, Deserialize, Getters, TypedBuilder)]
#[display(
    "{}: {}{} #{} @{}",
    name,
    unit_price,
    currency,
    quantity,
    transaction_date
)]
pub struct Item {
    /// The short name of the expense, e.g. `"Coffee"`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    name: String,
    /// The cost per item
    #[builder(setter(into))]
    #[getset(get = "pub")]
    unit_price: UnitPrice,
    /// The currency of the expense, e.g. `"EUR"`, this
    /// is the currency in which the expense was paid,
    /// and not necessarily the currency of the invoice.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    currency: Currency,
    /// The quantity of the expense, e.g. `2.0` for two items
    #[builder(setter(into))]
    #[getset(get = "pub")]
    quantity: Quantity,
    /// The date of the expense, e.g. `2025-05-31`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    transaction_date: Date,
}

impl Item {
    pub fn sample_expense_breakfast() -> Self {
        Self::builder()
            .name("Breakfast")
            .transaction_date(
                Date::builder()
                    .year(2025)
                    .month(Month::try_from(5).expect("LEQ 12"))
                    .day(Day::try_from(20).unwrap())
                    .build(),
            )
            .quantity(1.0)
            .unit_price(145.0)
            .currency(Currency::SEK)
            .build()
    }

    pub fn sample_expense_coffee() -> Self {
        Self::builder()
            .name("Coffee")
            .transaction_date(Date::sample())
            .quantity(2.0)
            .unit_price(4.0)
            .currency(Currency::GBP)
            .build()
    }

    pub fn sample_expense_sandwich() -> Self {
        Self::builder()
            .name("Sandwich")
            .transaction_date(Date::sample())
            .quantity(1.0)
            .unit_price(7.0)
            .currency(Currency::GBP)
            .build()
    }

    pub fn sample_consulting_service() -> Self {
        Self::builder()
            .name("Agreed Consulting Fees")
            .transaction_date(Date::sample())
            .quantity(22.0)
            .unit_price(500.0)
            .currency(Currency::EUR)
            .build()
    }
}

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

#[derive(Clone, Copy, Display, Debug, PartialEq, Serialize, Deserialize, From, Deref)]
pub struct Quantity(f64);

#[derive(Clone, Copy, Display, Debug, PartialEq, Default, Serialize, Deserialize, From, Deref)]
pub struct Cost(f64);

#[derive(Clone, Copy, Display, PartialEq, Debug, Serialize, Deserialize, From, Deref)]
pub struct UnitPrice(f64);

impl Item {
    pub fn converting_currency_and_calculating_total_cost(
        self,
        exchange_rates: &ExchangeRates,
    ) -> Result<ItemConvertedIntoTargetCurrency> {
        let converted_rates = self.with_exchange_rates(exchange_rates)?;
        Ok(converted_rates.with_total_cost())
    }

    fn with_total_cost(self) -> ItemConvertedIntoTargetCurrency {
        let cost = Cost::from(**self.quantity() * **self.unit_price());
        ItemConvertedIntoTargetCurrency::builder()
            .in_source_currency(self.clone())
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

impl FromStr for Item {
    type Err = crate::prelude::Error;

    /// Parses a string in the format: "name, unit_price, currency, quantity, transaction_date", or
    /// without spaces after commas, even mixed, e.g. "Coffee, 2.5,EUR, 3.0,2025-05-31".
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(',').map(str::trim).collect();
        if parts.len() != 5 {
            return Err(Error::InvalidExpenseItem {
                invalid_string: s.to_string(),
                reason: "Expected 5 comma-separated values".to_string(),
            });
        }

        let name = parts[0].to_string();
        let unit_price: UnitPrice = parts[1]
            .parse::<f64>()
            .map_err(|e| Error::InvalidExpenseItem {
                invalid_string: s.to_string(),
                reason: format!("Failed to parse unit_price: {e}"),
            })?
            .into();

        let currency = Currency::from_str(parts[2]).map_err(|e| Error::InvalidExpenseItem {
            invalid_string: s.to_string(),
            reason: format!("Failed to parse currency: {e}"),
        })?;

        let quantity: Quantity = parts[3]
            .parse::<f64>()
            .map_err(|e| Error::InvalidExpenseItem {
                invalid_string: s.to_string(),
                reason: format!("Failed to parse quantity: {e}"),
            })?
            .into();

        let transaction_date = Date::from_str(parts[4]).map_err(|e| Error::InvalidExpenseItem {
            invalid_string: s.to_string(),
            reason: format!("Failed to parse transaction_date: {e}"),
        })?;

        Ok(Item::builder()
            .name(name)
            .unit_price(unit_price)
            .currency(currency)
            .quantity(quantity)
            .transaction_date(transaction_date)
            .build())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_from_str() {
        // N.B. sometimes space after comma, sometimes not.
        let sut = Item::from_str("Coffee,2.5, EUR,3.0, 2025-05-31").expect("Failed to parse Item");
        assert_eq!(sut.name(), "Coffee");
        assert_eq!(**sut.unit_price(), 2.5);
        assert_eq!(sut.currency(), &Currency::EUR);
        assert_eq!(**sut.quantity(), 3.0);
        assert_eq!(
            sut.transaction_date(),
            &Date::from_str("2025-05-31").unwrap()
        );
    }
}
