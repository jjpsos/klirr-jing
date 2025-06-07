use chrono::{Datelike, NaiveDateTime};

use crate::prelude::*;

/// Information about this invoice, such as the identifier, date, purchase order,
/// and payment terms.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct InvoiceInformationWithoutDueDate {
    /// The unique identifier of this invoice, typically a number, e.g. `"90"`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    identifier: InvoiceIdentifier,
    /// A purchase order number associated with this invoice, e.g. `"PO-12345"`
    /// Typically agreed upon between the vendor and client before the
    /// invoice is issued.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    purchase_order: String,
    /// When this invoice was issued, e.g. `2025-05-31`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    date: Date,
    /// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    terms: PaymentTerms,

    /// E.g. "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation."
    #[builder(setter(into))]
    #[getset(get = "pub")]
    footer_text: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub struct InvoiceInformationWithDueDate {
    #[getset(get = "pub")]
    #[serde(flatten)]
    without_due_date: InvoiceInformationWithoutDueDate,

    #[getset(get = "pub")]
    due_date: Date,
}

impl From<NaiveDateTime> for Date {
    fn from(value: NaiveDateTime) -> Self {
        Self::builder()
            .year(value.year())
            .month(value.month())
            .day(value.day())
            .build()
    }
}

impl Date {
    pub fn to_datetime(&self) -> NaiveDateTime {
        let naive_date = chrono::NaiveDate::from_ymd_opt(
            **self.year() as i32,
            **self.month() as u32,
            **self.day() as u32,
        )
        .expect("Invalid date components");
        naive_date
            .and_hms_opt(0, 0, 0)
            .expect("Invalid time components")
    }

    pub fn advance_days(&self, days: &Day) -> Self {
        let datetime = self.to_datetime();
        let days: u8 = **days;
        let advanced_date = datetime + chrono::Duration::days(days as i64);
        Self::from(advanced_date)
    }

    pub fn advance(&self, terms: &PaymentTerms) -> Self {
        match terms {
            PaymentTerms::Net(days) => self.advance_days(days),
        }
    }
}

impl From<InvoiceInformationWithoutDueDate> for InvoiceInformationWithDueDate {
    fn from(value: InvoiceInformationWithoutDueDate) -> Self {
        let due_date = value.date.advance(value.terms());
        Self {
            without_due_date: value,
            due_date,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_advance() {
        let date = Date::from_str("2025-05-31").unwrap();
        let advanced = date.advance(&PaymentTerms::net30());
        assert_eq!(advanced, Date::from_str("2025-06-30").unwrap());
    }
}
