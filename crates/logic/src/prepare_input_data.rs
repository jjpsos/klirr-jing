use std::collections::HashMap;

use crate::prelude::*;

/// Response has format:
/// ```no_run
///  {
///      "amount": 1.0,
///      "base": "GBP",
///      "date": "2025-04-30",
///      "rates": {
/// 		    "EUR": 1.174
/// 	    }
///  }
/// ```
/// as given by `curl -s "https://api.frankfurter.app/2025-05-01?from=GBP&to=EUR"`
#[derive(Debug, Clone, Deserialize, Getters)]
struct FrankfurterApiResponse {
    #[getset(get = "pub")]
    rates: HashMap<Currency, f64>,
}

#[cfg(test)]
mod tests_frankfurter_api {
    use super::*;

    #[test]
    fn test_frankfurter_api_response() {
        let response = r#"{
            "amount": 1.0,
            "base": "GBP",
            "date": "2025-04-30",
            "rates": {
                "EUR": 1.174
            }
        }"#;

        let parsed: FrankfurterApiResponse = serde_json::from_str(response).unwrap();
        assert_eq!(
            parsed.rates.get(&Currency::EUR).unwrap().to_string(),
            "1.174"
        );
    }
}

/// Makes blocking requests to the Frankfurter API to get the exchange rate
fn get_exchange_rate(date: Date, from: Currency, to: Currency) -> Result<UnitPrice> {
    if from == to {
        return Ok(UnitPrice::from(1.0));
    }
    debug!(
        "Fetching exchange rate for {} from {} to {}",
        date, from, to
    );
    let url = format!("https://api.frankfurter.app/{}/{}/{}", date, from, to);
    // We will try to decode into `FrankfurterApiResponse`
    reqwest::blocking::get(&url)
        .map_err(|e| {
            let msg = format!("Failed to fetch exchange rate from {}: {}", url, e);
            error!("{}", msg);
            Error::NetworkError { underlying: msg }
        })?
        .json::<FrankfurterApiResponse>()
        .map_err(|e| {
            let msg = format!("Failed to parse exchange rate response: {}", e);
            error!("{}", msg);
            Error::ParseError { underlying: msg }
        })
        .and_then(|response| {
            response
                .rates()
                .get(&to)
                .cloned()
                .ok_or(Error::FoundNoExchangeRate {
                    target: to,
                    base: from,
                })
                .map(UnitPrice::from)
        })
}

fn get_exchange_rates_if_needed(
    items: &LineItemsWithoutCost,
) -> Result<HashMap<Currency, UnitPrice>> {
    let Ok(expenses) = items.clone().try_unwrap_expenses() else {
        debug!("No expenses found, skipping exchange rate fetching.");
        return Ok(HashMap::default());
    };
    debug!("Fetching exchanges rates for #{} expenses", expenses.len());
    let mut rates = HashMap::new();
    for expense in expenses {
        let from = *expense.currency();
        if !rates.contains_key(&from) {
            let to = *expense.currency();
            let date = *expense.transaction_date();
            let rate = get_exchange_rate(date, from, to)?;
            rates.insert(from, rate);
        }
    }
    Ok(rates)
}

pub fn prepare_invoice_input_data() -> Result<InvoiceInputDataToTypst> {
    // Prepare the data for the Typst source.
    // This is a placeholder function, you can add your own logic here.
    info!("Preparing invoice input data for PDF generation...");
    let input = InvoiceInputData::sample();
    let exchange_rates = get_exchange_rates_if_needed(input.line_items())?;
    let exchange_rates = ExchangeRates::builder()
        .target_currency(input.payment_info().currency().clone())
        .rates(exchange_rates)
        .build();
    let input = input.to_typst(exchange_rates)?;
    Ok(input)
}
