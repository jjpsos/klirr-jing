use crate::prelude::*;

/// Records periods of unpaid leave, each period spanning at least one month.
#[derive(Clone, Debug, Serialize, Deserialize, From, Deref)]
#[from(IndexSet<YearAndMonth>)]
pub struct MonthsOffRecord(IndexSet<YearAndMonth>);
impl MonthsOffRecord {
    pub fn new(periods: impl IntoIterator<Item = YearAndMonth>) -> Self {
        Self(IndexSet::from_iter(periods))
    }
    pub fn contains(&self, year_and_month: &YearAndMonth) -> bool {
        self.0.contains(year_and_month)
    }
}

impl MonthsOffRecord {
    pub fn sample() -> Self {
        Self::new([
            YearAndMonth::builder()
                .year(2020)
                .month(Month::try_from(9).expect("LEQ 12"))
                .build(),
            YearAndMonth::builder()
                .year(2021)
                .month(Month::try_from(3).expect("LEQ 12"))
                .build(),
        ])
    }
}
