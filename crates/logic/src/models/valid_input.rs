use crate::prelude::*;

#[derive(Debug, Clone, Display, TypedBuilder, Getters)]
#[display("Month: {}, out: {:?}, items: {}", month, maybe_output_path.as_ref().map(|d|d.display()), items)]
pub struct ValidInput {
    #[getset(get = "pub")]
    month: YearAndMonth,

    #[getset(get = "pub")]
    items: InvoicedItems,

    /// An optional override of where to save the output PDF file.
    #[builder(default, setter(into))]
    #[getset(get = "pub")]
    maybe_output_path: Option<PathBuf>,
}
