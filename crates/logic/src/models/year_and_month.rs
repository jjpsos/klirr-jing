use derive_more::Constructor;

use crate::prelude::*;

/// A date relevant for the invoice, e.g. invoice date, due date or a transaction
/// date for an expense.
#[derive(
    Clone,
    Copy,
    Debug,
    Display,
    PartialEq,
    Eq,
    Hash,
    SerializeDisplay,
    DeserializeFromStr,
    TypedBuilder,
    Getters,
    Constructor,
)]
#[display("{year:04}-{month:02}")]
pub struct YearAndMonth {
    /// e.g. 2025
    #[builder(setter(into))]
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    month: Month,
}

impl YearAndMonth {
    pub fn sample() -> Self {
        Self::builder()
            .year(2025)
            .month(Month::try_from(5).expect("LEQ 12"))
            .build()
    }
}

impl PartialOrd for YearAndMonth {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.year > other.year {
            Some(std::cmp::Ordering::Greater)
        } else if self.year < other.year {
            Some(std::cmp::Ordering::Less)
        } else if self.month > other.month {
            Some(std::cmp::Ordering::Greater)
        } else if self.month < other.month {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Equal)
        }
    }
}

impl std::str::FromStr for YearAndMonth {
    type Err = crate::prelude::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 2 {
            return Err(Error::FailedToParseDate {
                underlying: "Invalid Format YearAndMonth".to_owned(),
            });
        }

        let year = Year::from(
            parts[0]
                .parse::<i32>()
                .map_err(|_| Error::FailedToParseDate {
                    underlying: "Invalid year".to_owned(),
                })?,
        );
        let month =
            Month::try_from(
                parts[1]
                    .parse::<i32>()
                    .map_err(|_| Error::FailedToParseDate {
                        underlying: "Invalid month".to_owned(),
                    })?,
            )?;

        Ok(Self::builder().year(year).month(month).build())
    }
}
