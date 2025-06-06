use crate::prelude::*;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Error type for the logic crate, encapsulating various errors that can occur
/// during PDF generation and manipulation.
#[derive(Clone, Debug, ThisError)]
pub enum Error {
    /// Error when compiling Typst source to a PagedDocument.
    #[error("Failed to compile Typst source, because: {underlying}")]
    BuildPdf { underlying: String },

    /// Error when exporting a PagedDocument to PDF.
    #[error("Failed to export PagedDocument to PDF, because: {underlying}")]
    ExportDocumentToPdf { underlying: String },

    /// Error when saving the PDF to a file.
    #[error("Failed to save PDF, because: {underlying}")]
    SavePdf { underlying: String },
}
