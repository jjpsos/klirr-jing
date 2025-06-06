use crate::prelude::*;

use typst::layout::PagedDocument;
use typst_pdf::PdfOptions;
use typst_pdf::pdf;

pub fn render() -> Result<Pdf> {
    // Hardcoded Typst source string to compile.
    let source_text = "= Narnia";
    // Create a "World" (environment/context) for Typst compilation.
    let world = MinimalWorld::new(source_text);
    // Compile the Typst source into a PagedDocument (layouted pages).
    let compile_result = typst::compile::<PagedDocument>(&world);
    let doc = compile_result.output.map_err(|e| {
        let msg = format!("Failed to compile Typst source, because: {:?}", e);
        error!("{}", msg);
        Error::BuildPdf { underlying: msg }
    })?;
    // Export the PagedDocument to a PDF file.
    let export_pdf_options = &PdfOptions::default();
    let pdf_bytes = pdf(&doc, export_pdf_options).map_err(|e| {
        let msg = format!("Failed to export PDF, because: {:?}", e);
        error!("{}", msg);
        Error::ExportDocumentToPdf { underlying: msg }
    })?;
    // Convert the exported PDF bytes into a Pdf type.
    let pdf = Pdf::from(pdf_bytes);
    Ok(pdf)
}
