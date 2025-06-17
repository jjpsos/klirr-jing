use crate::prelude::*;

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
