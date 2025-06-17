use crate::prelude::*;

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

impl ProtoInvoiceInfo {
    pub fn sample() -> Self {
        Self::builder()
            .purchase_order(PurchaseOrder::sample())
            .terms(PaymentTerms::sample())
            .footer_text(
                "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation.",
            )
            .emphasize_color_hex(HexColor::sample())
            .offset(TimestampedInvoiceNumber::sample())
            .months_off_record(MonthsOffRecord::sample())
            .build()
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
