mod get_input;
mod init_logging;

pub mod prelude {
    pub(crate) use clap::{Parser, Subcommand};
    pub(crate) use derive_more::FromStr;
    pub(crate) use invoice_typst_render::prelude::*;
    pub(crate) use std::path::{Path, PathBuf};

    pub(crate) use crate::get_input::*;
}
use prelude::*;

/// Compile the Typst source into a PDF and safe it at the specified path.
fn create_pdf(input: ValidInput) -> Result<PathBuf> {
    let output_path = input.output_path();
    let data_from_disk = read_data_from_disk()?;
    let data = prepare_invoice_input_data(
        data_from_disk,
        input.month().year_and_month(),
        input.items(),
    )?;
    let l18n = get_localization()?;
    let pdf = render(Path::new("./crates/render/src/invoice.typ"), l18n, data)?;
    save_pdf(pdf, output_path)
}

/// Saves the PDF file to the specified path.
fn save_pdf(pdf: Pdf, pdf_name: impl AsRef<Path>) -> Result<PathBuf> {
    // now save the PDF to a file
    let output_path = PathBuf::from(pdf_name.as_ref());
    std::fs::write(&output_path, pdf.as_ref()).map_err(|e| {
        let msg = format!("Failed to write PDF to {}: {}", output_path.display(), e);
        error!("{}", msg);
        Error::SavePdf { underlying: msg }
    })?;
    Ok(output_path)
}

fn run() -> Result<PathBuf> {
    let input = get_input()?;
    info!("🔮 Starting PDF creation, input: {}...", input);
    create_pdf(input)
}

fn main() {
    init_logging::init_logging();
    match run() {
        Ok(path) => info!("✅ PDF created successfully at: {}", path.display()),
        Err(e) => error!("Error creating PDF: {}", e),
    }
}
