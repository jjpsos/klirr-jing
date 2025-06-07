use crate::prelude::*;

pub fn prepare_invoice_input_data() -> Result<InvoiceInputDataToTypst> {
    // Prepare the data for the Typst source.
    // This is a placeholder function, you can add your own logic here.
    info!("Preparing invoice input data for PDF generation...");
    let input = InvoiceInputData::sample();
    let exchange_rates = get_exchange_rates_if_needed(input.line_items())?;
    let input = input.to_typst(exchange_rates)?;
    Ok(input)
}
