use crate::prelude::*;

fn _get_client() -> Result<CompanyInformation> {
    deserialize_contents_of_ron(Path::new("./input/data/client.ron"))
}

fn _get_vendor() -> Result<CompanyInformation> {
    deserialize_contents_of_ron(Path::new("./input/data/vendor.ron"))
}

fn _get_payment_info() -> Result<PaymentInformation> {
    deserialize_contents_of_ron(Path::new("./input/data/payment.ron"))
}

fn _get_services_price() -> Result<ConsultingService> {
    deserialize_contents_of_ron(Path::new("./input/data/consulting_service.ron"))
}

fn _get_proto_invoice_info() -> Result<ProtoInvoiceInfo> {
    deserialize_contents_of_ron(Path::new("./input/data/invoice_info.ron"))
}

fn _get_expensed_months() -> Result<ExpensedMonths> {
    deserialize_contents_of_ron(Path::new("./input/data/expenses.ron"))
}

pub fn read_data_from_disk() -> Result<DataFromDisk> {
    // Read the input data from a file or other source.
    // This is a placeholder function, you can add your own logic here.
    info!("☑️ Reading data data...");
    let client = _get_client()?;
    let vendor = _get_vendor()?;
    let payment_info = _get_payment_info()?;
    let service_prices = _get_services_price()?;
    let proto_invoice_info = _get_proto_invoice_info()?;
    let expensed_months = _get_expensed_months()?;
    let input_data = DataFromDisk::builder()
        .client(client)
        .vendor(vendor)
        .payment_info(payment_info)
        .services_price(service_prices)
        .information(proto_invoice_info)
        .expensed_months(expensed_months)
        .build();
    Ok(input_data)
}
