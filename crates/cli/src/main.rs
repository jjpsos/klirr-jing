use std::{borrow::Cow, path::PathBuf};

use invoice_typst_render::prelude::*;

/// Saves tge PDF file to the specified path.
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

/// Compile the Typst source into a PDF and safe it at the specified path.
fn create_pdf<'s>(pdf_name: impl Into<Cow<'s, str>>) -> Result<PathBuf> {
    let pdf = render()?;
    save_pdf(pdf, pdf_name.into())
}

fn main() {
    match create_pdf("output.pdf") {
        Ok(path) => info!("✅ PDF created successfully at: {}", path.display()),
        Err(e) => error!("Error creating PDF: {}", e),
    }
}
