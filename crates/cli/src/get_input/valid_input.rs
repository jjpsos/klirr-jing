use crate::prelude::*;

#[derive(Debug, Clone, Display, TypedBuilder, Getters)]
#[display("Month: {}, out: {}, items: {}", month, output_path.display(), items)]
pub struct ValidInput {
    #[getset(get = "pub")]
    month: TargetMonth,

    #[getset(get = "pub")]
    items: InvoicedItems,

    #[getset(get = "pub")]
    output_path: PathBuf,
}
