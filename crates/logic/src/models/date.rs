use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::prelude::*;

/// A date relevant for the invoice, e.g. invoice date, due date or a transaction
/// date for an expense.
#[derive(
    Clone, Copy, Debug, Display, SerializeDisplay, DeserializeFromStr, TypedBuilder, Getters,
)]
#[display("{year:04}-{month:02}-{day:02}")]
pub struct Date {
    /// e.g. 2025
    #[builder(setter(into))]
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    month: Month,

    /// e.g. 31 for the last day of May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    day: Day,
}

impl std::str::FromStr for Date {
    type Err = crate::prelude::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() != 3 {
            return Err(Error::FailedToParseDate {
                underlying: "Invalid Format".to_owned(),
            });
        }

        let year = Year::from(
            parts[0]
                .parse::<i32>()
                .map_err(|_| Error::FailedToParseDate {
                    underlying: "Invalid year".to_owned(),
                })?,
        );
        let month = Month::from(
            parts[1]
                .parse::<i32>()
                .map_err(|_| Error::FailedToParseDate {
                    underlying: "Invalid month".to_owned(),
                })?,
        );
        let day = Day::from(
            parts[2]
                .parse::<i32>()
                .map_err(|_| Error::FailedToParseDate {
                    underlying: "Invalid day".to_owned(),
                })?,
        );

        Ok(Self { year, month, day })
    }
}
