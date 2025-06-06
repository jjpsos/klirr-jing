use crate::prelude::*;

pub type InvoiceInputData = InvoiceInputDataAbstract<LineItems>;
pub type InvoiceInputDataToTypst = InvoiceInputDataAbstract<LineItemsFlat>;

impl InvoiceInputData {
    pub fn to_typst(self) -> InvoiceInputDataToTypst {
        InvoiceInputDataToTypst {
            information: self.information,
            vendor: self.vendor,
            client: self.client,
            line_items: self.line_items.into(),
        }
    }
}

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct InvoiceInputDataAbstract<Items: Serialize> {
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
    line_items: Items,
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
