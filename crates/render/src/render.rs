use std::path::Path;

use crate::prelude::*;
use invoice_typst_logic::prelude::to_typst_let;
use typst::layout::PagedDocument;
use typst_pdf::PdfOptions;
use typst_pdf::pdf;

pub fn render(data: InvoiceInputDataToTypst) -> Result<Pdf> {
    // We will replace the place holder content in "crates/render/src/input.typ" with
    // the data from `data`
    // we start by reading the
    let typst_object_str = to_typst_let(&data);
    debug!("Typst invoice data source:\n{}", typst_object_str);
    // Create a "World" (environment/context) for Typst compilation.
    // let world = MinimalWorld::with_string_literal(source_text);
    let world =
        MinimalWorld::with_path(Path::new("./crates/render/src/mini.typ"), typst_object_str)?;

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
