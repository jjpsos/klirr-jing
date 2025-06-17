use crate::prelude::*;

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
