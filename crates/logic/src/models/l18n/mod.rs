use std::{collections::HashMap, path::Path};

use serde::de::DeserializeOwned;

use crate::prelude::*;

/// The language used and the content of the localization file.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18n {
    /// Which language this localization file is for, e.g.
    /// "en" for English
    #[builder(setter(into))]
    #[getset(get = "pub")]
    locale: String,
    /// The content of the localization file, which includes
    /// client information, invoice information, vendor information,
    /// and line items.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    content: L18nContent,
}
pub fn type_name<T>() -> String {
    std::any::type_name::<T>().to_string()
}
pub fn deserialize_contents_of_ron<T: DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    use ron::de::from_str;
    use std::fs;
    let path = path.as_ref();
    let ron_str = fs::read_to_string(path).map_err(|_| Error::FileNotFound {
        path: path.display().to_string(),
    })?;
    from_str(&ron_str).map_err(|e| Error::Deserialize {
        type_name: type_name::<T>(),
        error: e.to_string(),
    })
}
pub trait LoadRonFile {
    fn load_from_ron_file(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}
impl<T: DeserializeOwned> LoadRonFile for T {
    fn load_from_ron_file(path: impl AsRef<Path>) -> Result<Self> {
        deserialize_contents_of_ron(path)
    }
}

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

#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18nClientInfo {
    /// EN: "To:"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    to_company: String,

    /// EN: "VAT:"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    vat_number: String,
}

/// Localization for line items in the invoice, used in the
/// table of items being billed for.
#[derive(Debug, Clone, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct L18nLineItems {
    /// EN: "Item"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    description: String,

    /// EN: "When"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    when: String,

    /// EN: "Quantity"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    quantity: String,

    /// EN: "Unit price"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    unit_price: String,

    /// EN: "Total cost"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    total_cost: String,

    /// EN: "Grand Total:"
    #[builder(setter(into))]
    #[getset(get = "pub")]
    grand_total: String,
}

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

impl L18nVendorInfo {
    pub fn english() -> Self {
        Self::builder()
            .address("Address")
            .bank("Bank")
            .iban("IBAN")
            .bic("BIC")
            .organisation_number("Org. No.")
            .vat_number("VAT No.")
            .build()
    }
}

impl L18nLineItems {
    pub fn english() -> Self {
        Self::builder()
            .description("Item")
            .when("When")
            .quantity("Quantity")
            .unit_price("Unit price")
            .total_cost("Total cost")
            .grand_total("Grand Total:")
            .build()
    }
}
impl L18nInvoiceInfo {
    pub fn english() -> Self {
        Self::builder()
            .purchase_order("Purchase order:")
            .invoice_identifier("Invoice no:")
            .invoice_date("Invoice date:")
            .due_date("Due date:")
            .client_contact("For the attention of:")
            .vendor_contact("Our reference:")
            .terms("Terms")
            .build()
    }
}
impl L18nClientInfo {
    pub fn english() -> Self {
        Self::builder().to_company("To:").vat_number("VAT:").build()
    }
}
impl L18nContent {
    pub fn english() -> Self {
        Self::builder()
            .client_info(L18nClientInfo::english())
            .invoice_info(L18nInvoiceInfo::english())
            .vendor_info(L18nVendorInfo::english())
            .line_items(L18nLineItems::english())
            .month_names([
                "January".to_owned(),
                "February".to_owned(),
                "March".to_owned(),
                "April".to_owned(),
                "May".to_owned(),
                "June".to_owned(),
                "July".to_owned(),
                "August".to_owned(),
                "September".to_owned(),
                "October".to_owned(),
                "November".to_owned(),
                "December".to_owned(),
            ])
            .build()
    }
}
impl L18n {
    pub fn english() -> Self {
        Self::builder()
            .locale("en")
            .content(L18nContent::english())
            .build()
    }
}

lazy_static::lazy_static! {
    static ref L18N_MAP: HashMap<String, L18n> = {
        let mut m = HashMap::new();
        m.insert("en".to_string(), L18n::english());
        m
    };
}

impl L18n {
    pub fn new(locale: impl AsRef<str>) -> Result<Self> {
        let locale = locale.as_ref().to_owned();
        let Some(content) = L18N_MAP.get(&locale) else {
            return Err(Error::L18nNotFound { locale });
        };
        Ok(content.clone())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_ron_snapshot;

    use super::*;

    #[test]
    fn test_l18n_english() {
        assert_ron_snapshot!(&L18n::english());
    }
}
