use crate::prelude::*;

/// The content of the localization file, which includes
/// client information, invoice information, vendor information,
/// and line items.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18nContent {
    #[builder(setter(into))]
    #[getset(get = "pub")]
    client_info: L18nClientInfo,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    invoice_info: L18nInvoiceInfo,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    vendor_info: L18nVendorInfo,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    line_items: L18nLineItems,

    #[builder(setter(into))]
    #[getset(get = "pub")]
    month_names: [String; 12],
}
