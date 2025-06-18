use crate::prelude::*;

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

impl DataFromDisk {
    pub fn validate(self) -> Result<Self> {
        self.information.validate()?;
        Ok(self)
    }

    pub fn to_partial(self, input: ValidInput) -> Result<DataWithItemsPricedInSourceCurrency> {
        let items = input.items();
        let target_month = input.month();
        let invoice_date = target_month.to_date_end_of_month();
        let due_date = invoice_date.advance(self.information().terms());
        let is_expenses = items.is_expenses();
        let number = input.invoice_number(self.information());

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

        let input_unpriced =
            DataFromDiskWithItemsOfKind::<LineItemsPricedInSourceCurrency>::builder()
                .client(self.client)
                .information(full_info)
                .line_items(match items {
                    InvoicedItems::Service { days_off } => {
                        let working_days = working_days_in_month(
                            target_month,
                            self.information.months_off_record(),
                        )?;
                        let worked_days = working_days - days_off.map(|d| *d).unwrap_or(0);
                        let service = Item::builder()
                            .name(self.services_price.name().clone())
                            .transaction_date(invoice_date)
                            .quantity(Quantity::from(worked_days as f64))
                            .unit_price(*self.services_price.unit_price())
                            .currency(*self.payment_info.currency())
                            .build();
                        LineItemsPricedInSourceCurrency::Service(service)
                    }
                    InvoicedItems::Expenses => {
                        let expenses = get_expenses_for_month(
                            target_month,
                            self.expensed_months.expenses_for_months(),
                        )?;
                        LineItemsPricedInSourceCurrency::Expenses(expenses.clone())
                    }
                })
                .payment_info(self.payment_info)
                .vendor(self.vendor)
                .output_path(output_path)
                .build();

        Ok(input_unpriced)
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
    use test_log::test;

    #[test]
    fn test_serialization_sample() {
        assert_ron_snapshot!(DataFromDisk::sample())
    }
}
