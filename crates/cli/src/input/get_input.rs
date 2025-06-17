use crate::prelude::*;

/// The CLI arguments for generating an invoice PDF.
#[derive(Debug, Clone, TypedBuilder, Getters, Parser)]
#[command(name = "invoice")]
#[command(about = "Generate an invoice PDF", long_about = None)]
pub struct Input {
    /// The month for which the invoice is generated.
    #[arg(long, short = 'm', default_value_t = TargetMonth::Last)]
    #[getset(get = "pub")]
    month: TargetMonth,

    /// The language for which the invoice is generated.
    #[arg(long, short = 'l', default_value_t = Language::EN)]
    #[getset(get = "pub")]
    language: Language,

    /// The items to be invoiced, either expenses our consulting services
    /// with an optional number of days off.
    #[command(subcommand)]
    #[getset(get = "pub")]
    items: Option<TargetItems>,

    /// An optional override of where to save the output PDF file.
    #[arg(long, short = 'o')]
    out: Option<PathBuf>,
}

impl Input {
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

    pub fn parsed(self) -> Result<ValidInput> {
        if let Some(path) = &self.out {
            if !path
                .parent()
                .expect("unlikely you specified '/invoice.pdf'")
                .exists()
            {
                Err(Error::SpecifiedOutputPathDoesNotExist {
                    path: path.display().to_string(),
                })?;
            }
        }
        let items = self._invoiced_items()?;
        let valid = ValidInput::builder()
            .month(self.month.year_and_month())
            .items(items)
            .language(*self.language())
            .maybe_output_path(self.out)
            .build();
        Ok(valid)
    }
}

pub fn get_input() -> Result<ValidInput> {
    let input = Input::parse();
    input.parsed()
}
