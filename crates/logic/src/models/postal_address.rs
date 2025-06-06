use crate::prelude::*;

/// The postal address of a company
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct PostalAddress {
    /// The street address of a company, e.g.
    /// ```no_run
    /// "10 West Smithfield"
    /// "C/o Other company"
    /// "2nd floor"
    /// ```
    #[builder(setter(into))]
    #[getset(get = "pub")]
    street_address: StreetAddress,
    /// The zip code of the company, e.g. `"EC1A 1BB"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    zip: String,
    /// The country of the company, e.g. `"England"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    country: String,
    /// The city of the company, e.g. `"London"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    city: String,
}
