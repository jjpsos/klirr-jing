use std::{
    fs,
    path::{Path, PathBuf},
};

use derive_more::IsVariant;
use indexmap::IndexMap;

use crate::prelude::*;

pub type InputUnpriced = AbstractInput<LineItemsWithoutCost>;
pub type DataTypstCompat = AbstractInput<LineItemsFlat>;

pub const INVOICES_OUTPUT_DIR: &str = "invoices";

/// The items being invoiced this month, either services or expenses.
#[derive(Clone, Debug, Display, Serialize, Deserialize, IsVariant)]
pub enum InvoicedItems {
    #[display("Service {{ days_off: {} }} ", days_off.map(|d| *d).unwrap_or(0))]
    Service { days_off: Option<Day> },
    #[display("Expenses")]
    Expenses,
}
impl MaybeIsExpenses for InvoicedItems {
    fn is_expenses(&self) -> bool {
        self.is_expenses()
    }
}

use chrono::{Datelike, NaiveDate, Weekday};

fn working_days_in_month(
    target_month: &YearAndMonth,
    months_off_record: &MonthsOffRecord,
) -> Result<u8> {
    if months_off_record.contains(target_month) {
        return Err(Error::TargetMonthMustNotBeInRecordOfMonthsOff {
            target_month: *target_month,
        });
    }

    let year = **target_month.year() as i32;
    let month = **target_month.month() as u32;

    // Start from the 1st of the month
    let mut day = NaiveDate::from_ymd_opt(year, month, 1)
        .expect("Should always be able to create a date from year, month (and day)");

    // Get the last day of the month
    let last_day = if month == 12 {
        NaiveDate::from_ymd_opt(year + 1, 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    } else {
        NaiveDate::from_ymd_opt(year, month + 1, 1)
            .unwrap()
            .pred_opt()
            .unwrap()
    };

    let mut working_days = 0;
    while day <= last_day {
        match day.weekday() {
            Weekday::Mon | Weekday::Tue | Weekday::Wed | Weekday::Thu | Weekday::Fri => {
                working_days += 1;
            }
            _ => {}
        }
        day = day.succ_opt().unwrap();
    }

    Ok(working_days)
}

fn get_expenses_for_month(
    target_month: &YearAndMonth,
    expenses_for_months: &IndexMap<YearAndMonth, Vec<Item>>,
) -> Result<Vec<Item>> {
    if let Some(items) = expenses_for_months.get(target_month) {
        Ok(items.clone())
    } else {
        Err(Error::TargetMonthMustHaveExpenses {
            target_month: *target_month,
        })
    }
}

impl DataFromDisk {
    pub fn to_partial(self, input: ValidInput) -> Result<InputUnpriced> {
        let items = input.items();
        let target_month = input.month();
        let invoice_date = target_month.to_date_end_of_month();
        let due_date = invoice_date.advance(self.information().terms());
        let is_expenses = items.is_expenses();
        let number = calculate_invoice_number(
            self.information().offset(),
            target_month,
            is_expenses,
            self.information.months_off_record(),
        );

        let is_expenses_str_or_empty = if is_expenses { "_expenses" } else { "" };
        let vendor_name = self.vendor.company_name().replace(' ', "_");

        let output_path = input
            .maybe_output_path()
            .as_ref()
            .cloned()
            .map(OutputPath::AbsolutePath)
            .unwrap_or_else(|| {
                OutputPath::Name(format!(
                    "{}_{}{}_invoice_{}.pdf",
                    invoice_date, vendor_name, is_expenses_str_or_empty, number
                ))
            });

        let full_info = InvoiceInfoFull::builder()
            .due_date(due_date)
            .invoice_date(invoice_date)
            .emphasize_color_hex(self.information().emphasize_color_hex().clone())
            .footer_text(self.information().footer_text())
            .number(number)
            .purchase_order(self.information().purchase_order().clone())
            .terms(self.information().terms().clone())
            .build();

        let input_unpriced = AbstractInput::<LineItemsWithoutCost>::builder()
            .client(self.client)
            .information(full_info)
            .line_items(match items {
                InvoicedItems::Service { days_off } => {
                    let working_days =
                        working_days_in_month(target_month, self.information.months_off_record())?;
                    let worked_days = working_days - days_off.map(|d| *d).unwrap_or(0);
                    let service = Item::builder()
                        .name(self.services_price.name().clone())
                        .transaction_date(invoice_date)
                        .quantity(Quantity::from(worked_days as f64))
                        .unit_price(*self.services_price.unit_price())
                        .currency(*self.payment_info.currency())
                        .build();
                    LineItemsWithoutCost::Service(service)
                }
                InvoicedItems::Expenses => {
                    let expenses = get_expenses_for_month(
                        target_month,
                        &self.expensed_months.expenses_for_months,
                    )?;
                    LineItemsWithoutCost::Expenses(expenses.clone())
                }
            })
            .payment_info(self.payment_info)
            .vendor(self.vendor)
            .output_path(output_path)
            .build();

        Ok(input_unpriced)
    }
}

impl InputUnpriced {
    pub fn to_typst(self, exchange_rates_map: ExchangeRatesMap) -> Result<DataTypstCompat> {
        let exchange_rates = ExchangeRates::builder()
            .rates(exchange_rates_map)
            .target_currency(*self.payment_info().currency())
            .build();
        let line_items = LineItemsFlat::try_from((self.line_items, exchange_rates))?;
        Ok(DataTypstCompat {
            line_items,
            information: self.information,
            vendor: self.vendor,
            client: self.client,
            payment_info: self.payment_info,
            output_path: self.output_path,
        })
    }
}

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct DataFromDisk {
    /// Information about this specific invoice.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    information: ProtoInvoiceInfo,

    /// The company that issued the invoice, the vendor/seller/supplier/issuer.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    vendor: CompanyInformation,

    /// The company that pays the invoice, the customer/buyer.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    client: CompanyInformation,

    /// Payment information for the vendor, used for international transfers.
    /// This includes the IBAN, bank name, and BIC.
    /// This is used to ensure that the client can pay the invoice correctly.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    payment_info: PaymentInformation,

    /// Price of consulting service, if applicable.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    services_price: ConsultingService,

    /// Price of consulting service, if applicable.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    expensed_months: ExpensedMonths,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExpensedMonths {
    explanation: String,
    expenses_for_months: IndexMap<YearAndMonth, Vec<Item>>,
}
impl ExpensedMonths {
    pub fn new(expenses_for_months: IndexMap<YearAndMonth, Vec<Item>>) -> Self {
        Self {
            explanation: "Expenses for months".to_string(),
            expenses_for_months,
        }
    }

    pub fn contains(&self, month: &YearAndMonth) -> bool {
        self.expenses_for_months.contains_key(month)
    }
}

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct AbstractInput<Items: Serialize + MaybeIsExpenses> {
    /// Information about this specific invoice.
    #[builder(setter(into))]
    #[getset(get = "pub")]
    information: InvoiceInfoFull,

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

    /// Where to save the output PDF file.
    #[builder(setter(into))]
    output_path: OutputPath,
}

/// Returns the workspace root by going up from `cli` to the root.
fn workspace_root() -> PathBuf {
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_root
        .parent() // "../"
        .and_then(|p| p.parent()) // "../../"
        .map(PathBuf::from)
        .expect("Could not find workspace root from crate path")
}

/// Creates a folder at `WORKSPACE_ROOT/some/relative/path` if it doesn't exist.
fn create_folder_relative_to_workspace(path: impl AsRef<Path>) -> Result<PathBuf> {
    let workspace = workspace_root();
    let target_path = workspace.join(path.as_ref());
    let target_folder = target_path.parent().expect("Path should have a parent");
    if !target_folder.exists() {
        trace!(
            "Target folder: '{}' does not exist, creating now...",
            target_folder.display()
        );
        fs::create_dir_all(&target_path).map_err(|e| Error::FailedToCreateOutputDirectory {
            underlying: format!("{:?}", e),
        })?;
        trace!("Created target folder: '{}'", target_folder.display());
    }
    Ok(target_path)
}

impl<Items: Serialize + MaybeIsExpenses> AbstractInput<Items> {
    pub fn absolute_path(&self) -> Result<PathBuf> {
        match &self.output_path {
            OutputPath::AbsolutePath(path) => Ok(path.clone()),
            OutputPath::Name(name) => {
                let mut path = PathBuf::from(INVOICES_OUTPUT_DIR);
                path.push(name);
                create_folder_relative_to_workspace(&path)
            }
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum OutputPath {
    /// Manually overridden absolute path
    AbsolutePath(PathBuf),
    /// Relative path, automatically named
    Name(String),
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
        Self::builder()
            .year(2025)
            .month(Month::try_from(5).expect("LEQ 12 months"))
            .day(Day::try_from(31).expect("LEQ 31 days"))
            .build()
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
        Self::builder()
            .year(2025)
            .month(Month::try_from(5).expect("LEQ 12"))
            .build()
    }
}
impl TimestampedInvoiceNumber {
    pub fn sample() -> Self {
        Self::builder()
            .offset(237u16)
            .month(
                YearAndMonth::builder()
                    .year(2017)
                    .month(Month::try_from(3).expect("LEQ 12"))
                    .build(),
            )
            .build()
    }
}
impl MonthsOffRecord {
    pub fn sample() -> Self {
        Self::new([
            YearAndMonth::builder()
                .year(2020)
                .month(Month::try_from(9).expect("LEQ 12"))
                .build(),
            YearAndMonth::builder()
                .year(2021)
                .month(Month::try_from(3).expect("LEQ 12"))
                .build(),
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

impl Item {
    pub fn sample_expense_breakfast() -> Self {
        Self::builder()
            .name("Breakfast")
            .transaction_date(
                Date::builder()
                    .year(2025)
                    .month(Month::try_from(5).expect("LEQ 12"))
                    .day(Day::try_from(20).unwrap())
                    .build(),
            )
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

impl DataFromDisk {
    pub fn sample() -> Self {
        DataFromDisk::builder()
            .information(ProtoInvoiceInfo::sample())
            .client(CompanyInformation::sample_client())
            .vendor(CompanyInformation::sample_vendor())
            .payment_info(PaymentInformation::sample())
            .services_price(ConsultingService::sample())
            .expensed_months(ExpensedMonths::new(IndexMap::from_iter([(
                YearAndMonth::sample(),
                vec![
                    Item::sample_expense_breakfast(),
                    Item::sample_expense_coffee(),
                    Item::sample_expense_sandwich(),
                ],
            )])))
            .build()
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_ron_snapshot;

    use super::*;

    #[test]
    fn test_serialization_sample() {
        assert_ron_snapshot!(DataFromDisk::sample())
    }
}
