use crate::prelude::*;
use serde::{Deserialize, Serialize};

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct InvoiceInputData {
    /// Information about this specific invoice.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    information: InvoiceInformation,

    /// The company that issued the invoice, the vendor/seller/supplier/issuer.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    vendor: CompanyInformation,

    /// The company that pays the invoice, the customer/buyer.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    client: CompanyInformation,

    /// Services or expenses included in this invoice to be paid by the client.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    line_items: LineItems,
}

impl InvoiceInputData {
    /// A sample invoice input data for testing purposes.
    pub fn sample() -> Self {
        InvoiceInputData::builder()
            .information(
                InvoiceInformation::builder()
                    .identifier("INV-2025-001")
                    .purchase_order("PO-12345")
                    .date(Date::builder().year(2025).month(5).day(31).build())
                    .currency(Currency::EUR)
                    .terms(PaymentTerms::net30())
                    .build(),
            )
            .client(
                CompanyInformation::builder()
                    .company_name("Holmes Ltd")
                    .contact_person("Sherlock Holmes")
                    .organisation_number("9876543-2101")
                    .postal_address(
                        PostalAddress::builder()
                            .city("London")
                            .country("England")
                            .street_address(
                                StreetAddress::builder().line_1("221B Baker Street").build(),
                            )
                            .zip("NW1 6XE")
                            .build(),
                    )
                    .vat_number("GB987654321")
                    .build(),
            )
            .vendor(
                CompanyInformation::builder()
                    .company_name("Bra Detektiv AB")
                    .contact_person("Ture Sventon")
                    .organisation_number("556123-4567")
                    .postal_address(
                        PostalAddress::builder()
                            .city("Stockholm")
                            .country("Sweden")
                            .street_address(
                                StreetAddress::builder()
                                    .line_1("Storgatan 45")
                                    .line_2("4 tr")
                                    .line_3("Apt 12")
                                    .build(),
                            )
                            .zip("114 32")
                            .build(),
                    )
                    .vat_number("SE556123456701")
                    .build(),
            )
            .line_items(vec![
                Item::builder()
                    .name("Consulting services")
                    .transaction_date(Date::builder().year(2025).month(5).day(31).build())
                    .quantity(10.0)
                    .unit_price(50.0)
                    .currency(Currency::EUR)
                    .build(),
            ])
            .build()
    }
}

/// Services or expenses included in this invoice to be paid by the client.
#[derive(Clone, Debug, Serialize, Deserialize, From)]
#[from(Vec<Item>, Item)]
pub enum LineItems {
    /// Service sold by the vendor to the client, e.g. `"App development"`
    Service(Item),

    /// Expense incurred by the vendor, travel expenses for a conference/summit/
    /// retreat
    Expenses(Vec<Item>),
}

#[derive(Clone, Debug, Serialize, Deserialize, Getters, TypedBuilder)]
pub struct Item {
    /// The date of the expense, e.g. `2025-05-31`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    transaction_date: Date,

    /// The short name of the expense, e.g. `"Coffee"`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    name: String,

    /// The cost per item
    #[builder(setter(into))]
    #[getset(get = "pub")]
    unit_price: f64,

    /// The quantity of the expense, e.g. `2.0` for two items
    #[builder(setter(into))]
    #[getset(get = "pub")]
    quantity: f64,

    /// The currency of the expense, e.g. `"EUR"`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    currency: Currency,
}

/// A unique identifier for the invoice, e.g. `"90"`, `"INV-2025-001"`.
#[derive(Clone, Debug, Serialize, Deserialize, From, Deref)]
#[from(String, &'static str)]
#[serde(transparent)]
pub struct InvoiceIdentifier(String);

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

    /// The currency of this invoice, e.g. `EUR`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    currency: Currency,
}

/// The payment terms of this invoice, e.g. `Net { due_in: 30 }`
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PaymentTerms {
    /// Net payment due in a specific number of days, e.g. `Net(30)`
    Net(u8),
}
impl PaymentTerms {
    pub fn net30() -> Self {
        PaymentTerms::Net(30)
    }
}

/// A date relevant for the invoice, e.g. invoice date, due date or a transaction
/// date for an expense.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct Date {
    /// e.g. 2025
    #[builder(setter(into))]
    #[getset(get = "pub")]
    year: Year,

    /// e.g. 5 for May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    month: Month,

    /// e.g. 31 for the last day of May
    #[builder(setter(into))]
    #[getset(get = "pub")]
    day: Day,
}

/// Years since birth of Jesus christ, e.g. 2025
#[derive(Clone, Copy, Debug, Serialize, Deserialize, From, Deref)]
pub struct Year(u16);
impl From<i32> for Year {
    fn from(year: i32) -> Self {
        Self(year as u16)
    }
}

/// A month of the year, e.g. 1 for January, 2 for February, etc.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Deref)]
pub struct Month(u8);
impl From<i32> for Month {
    fn from(month: i32) -> Self {
        assert!(month >= 1 && month <= 12, "Month must be between 1 and 12");
        Self(month as u8)
    }
}

/// The day of the month, e.g. 1 for the first day, 31 for the last day of a month.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, From, Deref)]
pub struct Day(u8);
impl From<i32> for Day {
    fn from(day: i32) -> Self {
        assert!(day >= 1 && day <= 31, "Month must be between 1 and 31");
        Self(day as u8)
    }
}

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

/// Street address information
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct StreetAddress {
    /// The street address line 1, of the company, e.g. `"10 West Smithfield"`.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    line_1: String,

    /// The street address line 2, of the company, e.g. `"C/o Other company"`.
    #[builder(setter(into), default = "".to_owned())]
    #[getset(get = "pub")]
    line_2: String,

    /// The street address line 3, of the company, e.g. `"2nd floor"`.
    #[builder(setter(into), default = "".to_owned())]
    #[getset(get = "pub")]
    line_3: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_serialize() {
        assert_ron_snapshot!(InvoiceInputData::sample())
    }
}
