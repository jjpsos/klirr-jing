use crate::prelude::*;

#[derive(Debug, Clone, Copy, Display, Default, Parser, FromStr)]
#[command(name = "invoice")]
#[command(about = "Generate an invoice PDF", long_about = None)]
pub enum TargetMonth {
    Current,
    #[default]
    Last,
}
impl TargetMonth {
    pub fn year_and_month(&self) -> YearAndMonth {
        match self {
            TargetMonth::Current => YearAndMonth::current(),
            TargetMonth::Last => YearAndMonth::last(),
        }
    }
}
