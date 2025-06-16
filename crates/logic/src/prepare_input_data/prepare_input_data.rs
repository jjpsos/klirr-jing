use crate::prelude::*;

pub fn prepare_invoice_input_data(
    data_from_disk: DataFromDisk,
    input: ValidInput,
) -> Result<DataTypstCompat> {
    info!("Preparing invoice input data for PDF generation...");
    let partial = data_from_disk.to_partial(input)?;
    let exchange_rates = get_exchange_rates_if_needed(partial.line_items())?;
    let input_data = partial.to_typst(exchange_rates)?;
    info!("✅ Prepared invoice input data for PDF generation.");
    Ok(input_data)
}
