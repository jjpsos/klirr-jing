use crate::prelude::*;

pub fn prepare_invoice_input_data(
    input_data: InvoiceInputData,
    target_month: YearAndMonth,
) -> Result<InvoiceInputDataToTypst> {
    // Prepare the data for the Typst source.
    // This is a placeholder function, you can add your own logic here.
    info!("Preparing invoice input data for PDF generation...");
    let exchange_rates = get_exchange_rates_if_needed(input_data.line_items())?;
    let partial = input_data.to_partial(&target_month);
    let input_data = partial.to_typst(exchange_rates)?;
    Ok(input_data)
}
