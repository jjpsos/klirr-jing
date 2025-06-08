use std::path::Path;

use crate::prelude::*;
use invoice_typst_logic::prelude::to_typst_let;
use typst::layout::PagedDocument;
use typst_pdf::PdfOptions;
use typst_pdf::pdf;

pub fn render(data: InvoiceInputDataToTypst) -> Result<Pdf> {
    let typst_object_str = to_typst_let(&data);
    // Create a "World" (environment/context) for Typst compilation.
    let path_to_ui = Path::new("./crates/render/src/invoice.typ");
    let data_count = typst_object_str.len();

    debug!("☑️ Creating typst 'World' (environment/context), this usually takes ~2 seconds.");
    let world = MinimalWorld::with_path(path_to_ui, typst_object_str)?;
    debug!("✅ Created typst 'World' (environment/context)");

    // Compile the Typst source into a PagedDocument (layouted pages).
    debug!(
        "☑️ Compiling typst source at: {:?} with data: #{} bytes",
        path_to_ui, data_count
    );
    let compile_result = typst::compile::<PagedDocument>(&world);
    let doc = compile_result.output.map_err(|e| {
        let msg = format!("Failed to compile Typst source, because: {:?}", e);
        error!("{}", msg);
        Error::BuildPdf { underlying: msg }
    })?;
    debug!(
        "✅ Finished compiling typst source, output is #{} pages",
        doc.pages.len()
    );
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
