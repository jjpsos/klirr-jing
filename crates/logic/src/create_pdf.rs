use crate::prelude::*;

/// Compile the Typst source into a PDF and save it at the specified path.
pub fn create_pdf(
    input: ValidInput,
    render: impl Fn(L18n, DataTypstCompat) -> Result<Pdf>,
) -> Result<PathBuf> {
    let data_from_disk = read_data_from_disk()?;
    let l18n = get_localization(input.language())?;
    let data = prepare_invoice_input_data(data_from_disk, input)?;
    let output_path = data.absolute_path()?;
    let pdf = render(l18n, data)?;
    save_pdf(pdf, &output_path)?;
    Ok(output_path)
}

/// Saves the PDF file `pdf` to the specified path `pdf_path`.
fn save_pdf(pdf: Pdf, pdf_path: impl AsRef<Path>) -> Result<PathBuf> {
    info!("Saving PDF to: '{}'", pdf_path.as_ref().display());
    // now save the PDF to a file
    let output_path = PathBuf::from(pdf_path.as_ref());
    std::fs::write(&output_path, pdf.as_ref()).map_err(|e| {
        let msg = format!("Failed to write PDF to {}: {}", output_path.display(), e);
        error!("{}", msg);
        Error::SavePdf { underlying: msg }
    })?;
    info!("✅ Saved PDF to: '{}'", pdf_path.as_ref().display());
    Ok(output_path)
}
