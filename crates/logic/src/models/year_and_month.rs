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
    pub const fn january(year: u16) -> Self {
        Self::new(Year::new(year), Month::January)
    }
    pub const fn february(year: u16) -> Self {
        Self::new(Year::new(year), Month::February)
    }

    pub const fn march(year: u16) -> Self {
        Self::new(Year::new(year), Month::March)
    }

    pub const fn april(year: u16) -> Self {
        Self::new(Year::new(year), Month::April)
    }

    pub const fn may(year: u16) -> Self {
        Self::new(Year::new(year), Month::May)
    }

    pub const fn june(year: u16) -> Self {
        Self::new(Year::new(year), Month::June)
    }

    pub const fn july(year: u16) -> Self {
        Self::new(Year::new(year), Month::July)
    }

    pub const fn august(year: u16) -> Self {
        Self::new(Year::new(year), Month::August)
    }

    pub const fn september(year: u16) -> Self {
        Self::new(Year::new(year), Month::September)
    }

    pub const fn october(year: u16) -> Self {
        Self::new(Year::new(year), Month::October)
    }

    pub const fn november(year: u16) -> Self {
        Self::new(Year::new(year), Month::November)
    }

    pub const fn december(year: u16) -> Self {
        Self::new(Year::new(year), Month::December)
    }
}

impl YearAndMonth {
    pub fn sample() -> Self {
        Self::may(2025)
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
