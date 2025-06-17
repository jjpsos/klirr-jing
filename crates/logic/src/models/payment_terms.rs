use serde_with::{DeserializeFromStr, SerializeDisplay};

use crate::prelude::*;

/// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PaymentTerms {
    /// Net payment due in a specific number of days, e.g. `Net(30)`
    Net(NetDays),
}

impl PaymentTerms {
    pub fn net30() -> Self {
        PaymentTerms::Net(NetDays::net30())
    }
}

impl PaymentTerms {
    pub fn sample() -> Self {
        Self::net30()
    }
}

#[derive(
    Clone, Copy, Debug, SerializeDisplay, DeserializeFromStr, TypedBuilder, Getters, Display,
)]
#[display("Net {}", due_in)]
pub struct NetDays {
    /// The number of days until payment is due
    #[builder(setter(into))]
    #[getset(get = "pub")]
    due_in: Day,
}
impl FromStr for NetDays {
    type Err = crate::prelude::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let days = s
            .split("Net ")
            .nth(1)
            .ok_or(Error::FailedToParsePaymentTermsNetDays {
                invalid_string: s.to_owned(),
            })?;
        let days = Day::from_str(days).map_err(|_| Error::FailedToParsePaymentTermsNetDays {
            invalid_string: s.to_owned(),
        })?;
        Ok(Self::builder().due_in(days).build())
    }
}
impl NetDays {
    pub fn net30() -> Self {
        Self::builder()
            .due_in(Day::try_from(30).expect("LEQ 31 days"))
            .build()
    }
}
