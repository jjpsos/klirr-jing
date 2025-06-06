use crate::prelude::*;

/// Information about a company
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct CompanyInformation {
    /// Name of the person responsible for the invoice, e.g. `"John Doe"`.
    ///
    /// Used for "Our reference" in case of vendor, or
    /// "For attestation of", in case of client.
    #[builder(setter(into, strip_option))]
    #[getset(get = "pub")]
    contact_person: Option<String>,
    /// The unique organisation number of the company, e.g. `"123456789"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    organisation_number: String,
    /// The name of the company
    #[builder(setter(into))]
    #[getset(get = "pub")]
    company_name: String,
    /// The postal address of the company
    #[builder(setter(into))]
    #[getset(get = "pub")]
    postal_address: PostalAddress,
    /// The VAT number of the company, e.g. `"GB123456789"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    vat_number: String,
}
