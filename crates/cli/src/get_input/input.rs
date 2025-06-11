use crate::prelude::*;

/// The CLI arguments for generating an invoice PDF.
#[derive(Debug, Clone, TypedBuilder, Getters, Parser)]
#[command(name = "invoice")]
#[command(about = "Generate an invoice PDF", long_about = None)]
pub struct Input {
    /// The month for which the invoice is generated.
    #[arg(long)]
    #[getset(get = "pub")]
    month: Option<TargetMonth>,

    /// The items to be invoiced, either expenses our consulting services
    /// with an optional number of days off.
    #[command(subcommand)]
    #[getset(get = "pub")]
    items: Option<TargetItems>,

    /// The path to save the output PDF file.
    #[arg(long)]
    output_path: Option<PathBuf>,
}

impl Input {
    fn _output_path(&self) -> PathBuf {
        self.output_path
            .clone()
            .unwrap_or(PathBuf::from("output.pdf"))
    }

    fn _invoiced_items(&self) -> Result<InvoicedItems> {
        match self.items.clone().unwrap_or_default() {
            TargetItems::Ooo { days } => Ok(InvoicedItems::Service {
                days_off: if days == 0 {
                    None
                } else {
                    Some(Day::try_from(days)?)
                },
            }),
            TargetItems::Expenses => Ok(InvoicedItems::Expenses),
        }
    }

    pub fn parsed(&self) -> Result<ValidInput> {
        let items = self._invoiced_items()?;
        let valid = ValidInput::builder()
            .month(self.month.unwrap_or_default())
            .items(items)
            .output_path(self._output_path())
            .build();
        Ok(valid)
    }
}

pub fn get_input() -> Result<ValidInput> {
    let input = Input::parse();
    input.parsed()
}
