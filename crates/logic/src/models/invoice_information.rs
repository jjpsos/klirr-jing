use crate::prelude::*;

/// Information about this invoice, such as the identifier, date, purchase order,
/// and payment terms.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct InvoiceInformation {
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

}
