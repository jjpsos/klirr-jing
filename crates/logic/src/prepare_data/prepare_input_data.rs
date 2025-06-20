use crate::prelude::*;

pub fn prepare_invoice_input_data(
    data_from_disk: DataFromDisk,
    input: ValidInput,
    hardcoded_exchange_rates: Option<ExchangeRates>,
) -> Result<DataTypstCompat> {
    info!("Preparing invoice input data for PDF generation...");
    let partial = data_from_disk.to_partial(input)?;
    let exchange_rates = hardcoded_exchange_rates.map(Ok).unwrap_or_else(|| {
        get_exchange_rates_if_needed(*partial.payment_info().currency(), partial.line_items())
    })?;
    let data_typst_compat = partial.to_typst(exchange_rates)?;
    info!("✅ Prepared invoice input data for PDF generation.");
    Ok(data_typst_compat)
}
