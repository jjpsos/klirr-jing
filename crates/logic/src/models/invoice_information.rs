use chrono::{Datelike, NaiveDateTime};
use indexmap::IndexSet;

use crate::prelude::*;

/// An invoice number timestamp with year and month, e.g. `(237, 2025-05)`.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct TimestampedInvoiceNumber {
    /// A base offset for the invoice number, e.g. `237`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    offset: u16,

    /// The month and year for when the `offset` was used, e.g. `2025-05`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    month: YearAndMonth,
}

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

fn calculate_months_between(start: &YearAndMonth, end: &YearAndMonth) -> u16 {
    assert!(start < end, "Start month must be before end month");
    let start_year = **start.year();
    let start_month = **start.month() as u16;
    let end_year = **end.year();
    let end_month = **end.month() as u16;

    (end_year - start_year) * 12 + (end_month - start_month)
}

pub fn calculate_invoice_number(
    offset: &TimestampedInvoiceNumber,
    target_month: &YearAndMonth,
    is_expenses: bool,
    months_off_record: &MonthsOffRecord,
) -> InvoiceNumber {
    let months_elapsed_since_offset = calculate_months_between(&offset.month, target_month);

    let mut months_off_to_subtract = 0;
    for year_and_month in months_off_record.iter() {
        if year_and_month > &offset.month && year_and_month <= target_month {
            // If the month is off record, we need to adjust the invoice number
            // by subtracting the number of months off record.
            months_off_to_subtract += 1;
        }
    }
    let mut invoice_number = offset.offset + months_elapsed_since_offset - months_off_to_subtract;
    if is_expenses {
        // For expenses we add 1, ensuring that if we invoice for services and
        // expenses the same month, the expense invoice number is always higher.
        invoice_number += 1;
    }
    InvoiceNumber::from(invoice_number)
}

/// Information about this invoice, such as the number, date, purchase order,
/// and payment terms.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct ProtoInvoiceInfo {
    /// A purchase order number associated with this invoice, e.g. `"PO-12345"`
    /// Typically agreed upon between the vendor and client before the
    /// invoice is issued.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    purchase_order: PurchaseOrder,

    /// An offset which is used to calculate the invoice number, e.g. `(237, 2025-05)`.
    /// This is enables us to calculate the next invoice number based on the current
    /// date and this offset.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    offset: TimestampedInvoiceNumber,

    /// Record of months when we were 100% off, i.e. did not invoice for, e.g. `["2025-01", "2025-02"]`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    months_off_record: MonthsOffRecord,

    /// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    terms: PaymentTerms,

    /// E.g. "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation."
    #[builder(setter(into))]
    #[getset(get = "pub")]
    footer_text: String,

    /// Hex color code for the color emphasis of the invoice, e.g. `"#E6007A"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    emphasize_color_hex: HexColor,
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct InvoiceInfoFull {
    /// The unique number of this invoice, typically a number, e.g. `"90"`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    number: InvoiceNumber,

    /// When the payment is due, calculated from the invoice date and payment terms.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    invoice_date: Date,

    /// When the payment is due, calculated from the invoice date and payment terms.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    due_date: Date,

    /// A purchase order number associated with this invoice, e.g. `"PO-12345"`
    /// Typically agreed upon between the vendor and client before the
    /// invoice is issued.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    purchase_order: PurchaseOrder,

    /// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    terms: PaymentTerms,

    /// E.g. "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation."
    #[builder(setter(into))]
    #[getset(get = "pub")]
    footer_text: String,

    /// Hex color code for the color emphasis of the invoice, e.g. `"#E6007A"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    emphasize_color_hex: HexColor,
}

impl From<NaiveDateTime> for Date {
    fn from(value: NaiveDateTime) -> Self {
        Self::builder()
            .year(value.year())
            .month(
                Month::try_from(value.month())
                    .expect("NativeDateTime should always return valid month."),
            )
            .day(
                Day::try_from(value.day()).expect("NativeDateTime should always return valid day."),
            )
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
            PaymentTerms::Net(days) => self.advance_days(days.due_in()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_advance() {
        let date = Date::from_str("2025-05-31").unwrap();
        let advanced = date.advance(&PaymentTerms::net30());
        assert_eq!(advanced, Date::from_str("2025-06-30").unwrap());
    }
}
