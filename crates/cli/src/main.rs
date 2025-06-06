mod init_logging;

use invoice_typst_render::prelude::*;
use std::{borrow::Cow, path::PathBuf};

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

fn prepare_invoice_input_data() -> Result<InvoiceInputData> {
    // Prepare the data for the Typst source.
    // This is a placeholder function, you can add your own logic here.
    info!("Preparing invoice input data for PDF generation...");
   let input = InvoiceInputData::sample();
    Ok(input)
}

/// Compile the Typst source into a PDF and safe it at the specified path.
fn create_pdf<'s>(pdf_name: impl Into<Cow<'s, str>>) -> Result<PathBuf> {
    let data = prepare_invoice_input_data()?;
    let pdf = render(data.to_typst())?;
    save_pdf(pdf, pdf_name.into())
}

fn main() {
    init_logging::init_logging();
    info!("🔮 Starting PDF creation...");
    match create_pdf("output.pdf") {
        Ok(path) => info!("✅ PDF created successfully at: {}", path.display()),
        Err(e) => error!("Error creating PDF: {}", e),
    }
}
