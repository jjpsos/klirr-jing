use crate::prelude::*;

/// Localization for invoice information, such as purchase order,
/// invoice number, dates, and terms.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18nInvoiceInfo {
    /// EN: "Purchase order:"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    purchase_order: String,

    /// EN: "Invoice no:"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    invoice_identifier: String,

    /// EN: "Invoice date:"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    invoice_date: String,

    /// EN: "Due date:"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    due_date: String,

    /// EN: "For the attention of:"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    client_contact: String,

    /// EN: "Our reference:"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    vendor_contact: String,

    /// EN: "Terms"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    terms: String,
}
