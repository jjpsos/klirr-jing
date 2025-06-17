use crate::prelude::*;

/// Localization for vendor information in the invoice,
/// such as bank details and organization information.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18nVendorInfo {
    /// EN: "Address"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    address: String,

    /// EN: "Bank"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    bank: String,

    /// EN: "IBAN"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    iban: String,

    /// EN: "BIC"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    bic: String,

    /// EN: "Org. No."
    #[builder(setter(into))]
    #[getset(get = "pub")]
    organisation_number: String,

    /// EN: "VAT No."
    #[builder(setter(into))]
    #[getset(get = "pub")]
    vat_number: String,
}
