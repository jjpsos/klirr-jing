use crate::prelude::*;

fn client() -> Result<CompanyInformation> {
    deserialize_contents_of_ron(directory_relative_workspace_with_path_components(
        "./input/data/client.ron",
    ))
}

fn vendor() -> Result<CompanyInformation> {
    deserialize_contents_of_ron(directory_relative_workspace_with_path_components(
        "./input/data/vendor.ron",
    ))
}

fn payment_info() -> Result<PaymentInformation> {
    deserialize_contents_of_ron(directory_relative_workspace_with_path_components(
        "./input/data/payment.ron",
    ))
}

fn services_price() -> Result<ConsultingService> {
    deserialize_contents_of_ron(directory_relative_workspace_with_path_components(
        "./input/data/consulting_service.ron",
    ))
}

fn proto_invoice_info() -> Result<ProtoInvoiceInfo> {
    deserialize_contents_of_ron(directory_relative_workspace_with_path_components(
        "./input/data/invoice_info.ron",
    ))
}

fn expensed_months() -> Result<ExpensedMonths> {
    deserialize_contents_of_ron(directory_relative_workspace_with_path_components(
        "./input/data/expenses.ron",
    ))
}

pub fn read_data_from_disk() -> Result<DataFromDisk> {
    // Read the input data from a file or other source.
    // This is a placeholder function, you can add your own logic here.
    debug!("☑️ Reading data from disk...");
    let client = client()?;
    let vendor = vendor()?;
    let payment_info = payment_info()?;
    let service_prices = services_price()?;
    let proto_invoice_info = proto_invoice_info()?;
    let expensed_months = expensed_months()?;
    let input_data = DataFromDisk::builder()
        .client(client)
        .vendor(vendor)
        .payment_info(payment_info)
        .services_price(service_prices)
        .information(proto_invoice_info)
        .expensed_months(expensed_months)
        .build();
    debug!("✅ Read data from disk!");
    input_data.validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_read_data_from_disk() {
        let result = read_data_from_disk().unwrap();
        assert_eq!(*result.payment_info().currency(), Currency::EUR);
    }
}
