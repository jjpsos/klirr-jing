mod init_logging;

use invoice_typst_render::prelude::*;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

/// Saves the PDF file to the specified path.
fn save_pdf(pdf: Pdf, pdf_name: Cow<str>) -> Result<PathBuf> {
    // now save the PDF to a file
    let output_path = PathBuf::from(pdf_name.as_ref());
    std::fs::write(&output_path, pdf.as_ref()).map_err(|e| {
        let msg = format!("Failed to write PDF to {}: {}", output_path.display(), e);
        error!("{}", msg);
        Error::SavePdf { underlying: msg }
    })?;
    Ok(output_path)
}

fn get_localization() -> Result<L18n> {
    // Read the input data from a file or other source.
    // This is a placeholder function, you can add your own logic here.
    info!("☑️ Reading localisation data...");
    warn!("Using sample data for demonstration purposes.");
    let l18n = L18n::english();
    Ok(l18n)
}

fn read_input_data() -> Result<ProtoInput> {
    // Read the input data from a file or other source.
    // This is a placeholder function, you can add your own logic here.
    info!("☑️ Reading invoice input data...");
    warn!("Using sample data for demonstration purposes.");
    let input_data = ProtoInput::sample();
    Ok(input_data)
}

/// Compile the Typst source into a PDF and safe it at the specified path.
fn create_pdf<'s>(
    pdf_name: impl Into<Cow<'s, str>>,
    target_month: YearAndMonth,
    invoiced_items: InvoicedItems,
) -> Result<PathBuf> {
    let input_data = read_input_data()?;
    let data = prepare_invoice_input_data(input_data, target_month, invoiced_items)?;
    let l18n = get_localization()?;
    let pdf = render(Path::new("./crates/render/src/invoice.typ"), l18n, data)?;
    save_pdf(pdf, pdf_name.into())
}

fn main() {
    init_logging::init_logging();
    let target_month = YearAndMonth::builder().year(2025).month(5).build();
    warn!("Using hardcoded target month: {}", target_month);
    let invoiced_items = InvoicedItems::Service { days_off: 3 };
    warn!("Using hardcoded invoiced items: {:?}", invoiced_items);
    info!("🔮 Starting PDF creation...");
    match create_pdf("output.pdf", target_month, invoiced_items) {
        Ok(path) => info!("✅ PDF created successfully at: {}", path.display()),
        Err(e) => error!("Error creating PDF: {}", e),
    }
}
