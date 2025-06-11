use crate::prelude::*;

pub fn prepare_invoice_input_data(
    input_data: ProtoInput,
    target_month: YearAndMonth,
    invoiced_items: InvoicedItems,
) -> Result<InputTypstFormat> {
    // Prepare the data for the Typst source.
    // This is a placeholder function, you can add your own logic here.
    info!("Preparing invoice input data for PDF generation...");
    let partial = input_data.to_partial(&target_month, &invoiced_items)?;
    let exchange_rates = get_exchange_rates_if_needed(partial.line_items())?;
    let input_data = partial.to_typst(exchange_rates)?;
    Ok(input_data)
}
