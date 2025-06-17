use crate::prelude::*;

#[derive(Debug, Clone, Display, TypedBuilder, Getters)]
#[display("Month: {}, out: {:?}, items: {}, language: {}", month, maybe_output_path.as_ref().map(|d|d.display()), items, language)]
pub struct ValidInput {
    #[getset(get = "pub")]
    language: Language,

    #[getset(get = "pub")]
    month: YearAndMonth,

    #[getset(get = "pub")]
    items: InvoicedItems,

    /// An optional override of where to save the output PDF file.
    #[builder(default, setter(into))]
    #[getset(get = "pub")]
    maybe_output_path: Option<PathBuf>,
}
