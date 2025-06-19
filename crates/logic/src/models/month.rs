use crate::prelude::*;

/// A month of the year, e.g. 1 for January, 2 for February, etc.
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Display, Serialize, Deserialize)]
#[display("{}", self.month())]
pub enum Month {
    January = 1,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl std::fmt::Debug for Month {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.month())
    }
}

impl Month {
    pub fn month(&self) -> &u8 {
        match self {
            Month::January => &1,
            Month::February => &2,
            Month::March => &3,
            Month::April => &4,
            Month::May => &5,
            Month::June => &6,
            Month::July => &7,
            Month::August => &8,
            Month::September => &9,
            Month::October => &10,
            Month::November => &11,
            Month::December => &12,
        }
    }
}
impl std::ops::Deref for Month {
    type Target = u8;
    fn deref(&self) -> &Self::Target {
        self.month()
    }
}

impl TryFrom<i32> for Month {
    type Error = crate::prelude::Error;
    fn try_from(month: i32) -> Result<Self> {
        match month {
            1 => Ok(Month::January),
            2 => Ok(Month::February),
            3 => Ok(Month::March),
            4 => Ok(Month::April),
            5 => Ok(Month::May),
            6 => Ok(Month::June),
            7 => Ok(Month::July),
            8 => Ok(Month::August),
            9 => Ok(Month::September),
            10 => Ok(Month::October),
            11 => Ok(Month::November),
            12 => Ok(Month::December),
            _ => Err(Error::InvalidMonth {
                month,
                reason: "Month must be between 1 and 12".to_string(),
            }),
        }
    }
}

impl TryFrom<u8> for Month {
    type Error = crate::prelude::Error;
    fn try_from(month: u8) -> Result<Self> {
        Self::try_from(month as i32)
    }
}

impl TryFrom<u32> for Month {
    type Error = crate::prelude::Error;
    fn try_from(month: u32) -> Result<Self> {
        Self::try_from(month as i32)
    }
}
