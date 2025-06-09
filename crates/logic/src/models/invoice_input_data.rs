use crate::prelude::*;

pub type InvoiceInputData = InvoiceInputDataAbstract<ProtoInvoiceInfo, LineItemsWithoutCost>;

pub type InvoiceInputDataToTypst = InvoiceInputDataAbstract<InvoiceInfoFull, LineItemsFlat>;

pub type InvoiceInputDataPartial = InvoiceInputDataAbstract<InvoiceInfoFull, LineItemsWithoutCost>;

impl InvoiceInputData {
    pub fn to_partial(self, target_month: &YearAndMonth) -> InvoiceInputDataPartial {
        let invoice_date = target_month.to_date_end_of_month();
        let due_date = invoice_date.advance(self.information().terms());
        let is_expenses = self.line_items.is_expenses();
        let number = calculate_invoice_number(
            self.information().offset(),
            target_month,
            is_expenses,
            self.information.months_off_record(),
        );
        let full_info = InvoiceInfoFull::builder()
            .due_date(due_date)
            .invoice_date(invoice_date)
            .emphasize_color_hex(self.information().emphasize_color_hex().clone())
            .footer_text(self.information().footer_text())
            .number(number)
            .purchase_order(self.information().purchase_order().clone())
            .terms(self.information().terms().clone())
            .build();
        InvoiceInputDataAbstract::<InvoiceInfoFull, LineItemsWithoutCost>::builder()
            .client(self.client)
            .information(full_info)
            .line_items(self.line_items)
            .payment_info(self.payment_info)
            .vendor(self.vendor)
            .build()
    }
}

impl InvoiceInputDataPartial {
    pub fn to_typst(self, exchange_rates_map: ExchangeRatesMap) -> Result<InvoiceInputDataToTypst> {
        let exchange_rates = ExchangeRates::builder()
            .rates(exchange_rates_map)
            .target_currency(*self.payment_info().currency())
            .build();
        let line_items = LineItemsFlat::try_from((self.line_items, exchange_rates))?;
        Ok(InvoiceInputDataToTypst {
            line_items,
            information: self.information,
            vendor: self.vendor,
            client: self.client,
            payment_info: self.payment_info,
        })
    }
}

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct InvoiceInputDataAbstract<Info: Serialize, Items: Serialize + IsExpenses> {
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

impl Date {
    pub fn sample() -> Self {
        Self::builder().year(2025).month(5).day(31).build()
    }
}
impl PaymentTerms {
    pub fn sample() -> Self {
        Self::net30()
    }
}
impl InvoiceNumber {
    pub fn sample() -> Self {
        Self::from(9876)
    }
}
impl PurchaseOrder {
    pub fn sample() -> Self {
        Self::from("PO-12345")
    }
}
impl HexColor {
    pub fn sample() -> Self {
        Self::from_str("#E6007A").expect("Failed to create sample HexColor")
    }
}
impl YearAndMonth {
    pub fn sample() -> Self {
        Self::builder().year(2025).month(5).build()
    }
}
impl TimestampedInvoiceNumber {
    pub fn sample() -> Self {
        Self::builder()
            .offset(237u16)
            .month(YearAndMonth::builder().year(2017).month(3).build())
            .build()
    }
}
impl MonthsOffRecord {
    pub fn sample() -> Self {
        Self::new([
            YearAndMonth::builder().year(2020).month(9).build(),
            YearAndMonth::builder().year(2021).month(3).build(),
        ])
    }
}
impl ProtoInvoiceInfo {
    pub fn sample() -> Self {
        Self::builder()
            .purchase_order(PurchaseOrder::sample())
            .terms(PaymentTerms::sample())
            .footer_text(
                "Reverse VAT according to chapter 1 2§ first section 4b in the VAT regulation.",
            )
            .emphasize_color_hex(HexColor::sample())
            .offset(TimestampedInvoiceNumber::sample())
            .months_off_record(MonthsOffRecord::sample())
            .build()
    }
}

impl PostalAddress {
    pub fn sample_client() -> Self {
        Self::builder()
            .city("London")
            .country("England")
            .street_address(StreetAddress::builder().line_1("221B Baker Street").build())
            .zip("NW1 6XE")
            .build()
    }

    pub fn sample_vendor() -> Self {
        Self::builder()
            .city("Stockholm")
            .country("Sweden")
            .street_address(
                StreetAddress::builder()
                    .line_1("Storgatan 45")
                    .line_2("4 tr")
                    .build(),
            )
            .zip("114 32")
            .build()
    }
}

impl CompanyInformation {
    pub fn sample_client() -> Self {
        Self::builder()
            .company_name("Holmes Ltd")
            .contact_person("Sherlock Holmes")
            .organisation_number("9876543-2101")
            .postal_address(PostalAddress::sample_client())
            .vat_number("GB987654321")
            .build()
    }

    pub fn sample_vendor() -> Self {
        Self::builder()
            .company_name("Bra Detektiv AB")
            .contact_person("Ture Sventon")
            .organisation_number("556123-4567")
            .postal_address(PostalAddress::sample_vendor())
            .vat_number("SE556123456701")
            .build()
    }
}

impl ItemWithoutCost {
    pub fn sample_expense_breakfast() -> Self {
        Self::builder()
            .name("Breakfast")
            .transaction_date(Date::builder().year(2025).month(5).day(20).build())
            .quantity(1.0)
            .unit_price(145.0)
            .currency(Currency::SEK)
            .build()
    }

    pub fn sample_expense_coffee() -> Self {
        Self::builder()
            .name("Coffee")
            .transaction_date(Date::sample())
            .quantity(2.0)
            .unit_price(4.0)
            .currency(Currency::GBP)
            .build()
    }

    pub fn sample_expense_sandwich() -> Self {
        Self::builder()
            .name("Sandwich")
            .transaction_date(Date::sample())
            .quantity(1.0)
            .unit_price(7.0)
            .currency(Currency::GBP)
            .build()
    }

    pub fn sample_consulting_service() -> Self {
        Self::builder()
            .name("Agreed Consulting Fees")
            .transaction_date(Date::sample())
            .quantity(22.0)
            .unit_price(500.0)
            .currency(Currency::EUR)
            .build()
    }
}

impl PaymentInformation {
    pub fn sample() -> Self {
        Self::builder()
            .bank_name("SEB")
            .iban("SE21 9000 0123 9876 5432 1009")
            .bic("ESSESESS")
            .currency(Currency::EUR)
            .build()
    }
}

impl InvoiceInputData {
    pub fn sample_expenses() -> Self {
        InvoiceInputData::builder()
            .information(ProtoInvoiceInfo::sample())
            .client(CompanyInformation::sample_client())
            .vendor(CompanyInformation::sample_vendor())
            .line_items(vec![
                ItemWithoutCost::sample_expense_breakfast(),
                ItemWithoutCost::sample_expense_coffee(),
                ItemWithoutCost::sample_expense_sandwich(),
            ])
            .payment_info(PaymentInformation::sample())
            .build()
    }

    pub fn sample_consulting_services() -> Self {
        InvoiceInputData::builder()
            .information(ProtoInvoiceInfo::sample())
            .client(CompanyInformation::sample_client())
            .vendor(CompanyInformation::sample_vendor())
            .line_items(ItemWithoutCost::sample_consulting_service())
            .payment_info(PaymentInformation::sample())
            .build()
    }
}
