use crate::prelude::*;

pub type InvoiceInputData =
    InvoiceInputDataAbstract<InvoiceInformationWithoutDueDate, LineItemsWithoutCost>;

pub type InvoiceInputDataToTypst =
    InvoiceInputDataAbstract<InvoiceInformationWithDueDate, LineItemsFlat>;

impl InvoiceInputData {
    pub fn to_typst(self, exchange_rates_map: ExchangeRatesMap) -> Result<InvoiceInputDataToTypst> {
        let exchange_rates = ExchangeRates::builder()
            .rates(exchange_rates_map)
            .target_currency(*self.payment_info().currency())
            .build();
        let line_items = LineItemsFlat::try_from((self.line_items, exchange_rates))?;
        let information_with_due_date = InvoiceInformationWithDueDate::from(self.information);
        Ok(InvoiceInputDataToTypst {
            line_items,
            information: information_with_due_date,
            vendor: self.vendor,
            client: self.client,
            payment_info: self.payment_info,
        })
    }
}

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct InvoiceInputDataAbstract<Info: Serialize, Items: Serialize> {
    /// Information about this specific invoice.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    information: Info,

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

    /// Payment information for the vendor, used for international transfers.
    /// This includes the IBAN, bank name, and BIC.
    /// This is used to ensure that the client can pay the invoice correctly.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    payment_info: PaymentInformation,
}

/// Bank account details for the vendor, used for international transfers.
/// This includes the IBAN, bank name, and BIC.
/// This is used to ensure that the client can pay the invoice correctly.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct PaymentInformation {
    /// The IBAN (International Bank Account Number) of the vendor's bank account,
    #[builder(setter(into))]
    #[getset(get = "pub")]
    iban: String,

    /// The name of the vendor's bank, used for international transfers.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    bank_name: String,

    /// The BIC (Bank Identifier Code) of the vendor's bank, used for international
    #[builder(setter(into))]
    #[getset(get = "pub")]
    bic: String,

    /// The currency of this invoice, e.g. `EUR`
    #[builder(setter(into))]
    #[getset(get = "pub")]
    currency: Currency,
}

impl InvoiceInputData {
    /// A sample invoice input data for testing purposes.
    pub fn sample() -> Self {
        InvoiceInputData::builder()
            .information(
                InvoiceInformationWithoutDueDate::builder()
                    .identifier("INV-9876")
                    .purchase_order("PO-12345")
                    .date(Date::builder().year(2025).month(5).day(31).build())
                    .terms(PaymentTerms::net30())
                    .footer_text("Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation.")
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
                                    .build(),
                            )
                            .zip("114 32")
                            .build(),
                    )
                    .vat_number("SE556123456701")
                    .build(),
            )
            .line_items(vec![
                ItemWithoutCost::builder()
                    .name("Breakfast")
                    .transaction_date(Date::builder().year(2025).month(5).day(20).build())
                    .quantity(1.0)
                    .unit_price(145.0)
                    .currency(Currency::SEK)
                    .build(),
                ItemWithoutCost::builder()
                    .name("Coffee")
                    .transaction_date(Date::builder().year(2025).month(5).day(31).build())
                    .quantity(2.0)
                    .unit_price(4.0)
                    .currency(Currency::GBP)
                    .build(),
                ItemWithoutCost::builder()
                    .name("Sandwich")
                    .transaction_date(Date::builder().year(2025).month(5).day(31).build())
                    .quantity(1.0)
                    .unit_price(7.0)
                    .currency(Currency::GBP)
                    .build(),
            ])
            .payment_info(
                PaymentInformation::builder()
                    .bank_name("SEB")
                    .iban("SE21 9000 0123 9876 5432 1009")
                    .bic("ESSESESS")
                    .currency(Currency::EUR)
                    .build(),
            )
            .build()
    }
}
