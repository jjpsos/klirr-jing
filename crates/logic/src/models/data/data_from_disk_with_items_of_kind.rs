use std::fs;

pub const INVOICES_OUTPUT_DIR: &str = "invoices";

use crate::prelude::*;

pub type DataWithItemsPricedInSourceCurrency =
    DataFromDiskWithItemsOfKind<LineItemsPricedInSourceCurrency>;
pub type DataTypstCompat = DataFromDiskWithItemsOfKind<LineItemsFlat>;

pub trait HasSample: Sized {
    /// Returns a sample instance of the type.
    fn sample() -> Self;
}

impl<Items: Serialize + MaybeIsExpenses + HasSample> HasSample
    for DataFromDiskWithItemsOfKind<Items>
{
    fn sample() -> Self {
        Self::builder()
            .information(InvoiceInfoFull::sample())
            .vendor(CompanyInformation::sample())
            .client(CompanyInformation::sample())
            .line_items(Items::sample())
            .payment_info(PaymentInformation::sample())
            .output_path(OutputPath::Name("invoice.pdf".into()))
            .build()
    }
}

/// The input data for the invoice, which includes information about the invoice,
/// the vendor, and the client and the products/services included in the invoice.
#[derive(Clone, Debug, Serialize, Deserialize, TypedBuilder, Getters)]
pub struct DataFromDiskWithItemsOfKind<Items: Serialize + MaybeIsExpenses> {
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
pub fn workspace_root() -> PathBuf {
    let crate_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    crate_root
        .parent() // "../"
        .and_then(|p| p.parent()) // "../../"
        .map(PathBuf::from)
        .expect("Could not find workspace root from crate path")
}

/// Forms a path relative to the workspace root, e.g. `WORKSPACE_ROOT/some/relative/path`.
pub fn directory_relative_workspace_with_path_components(path: impl AsRef<Path>) -> PathBuf {
    let workspace = workspace_root();
    workspace.join(path.as_ref())
}

/// Creates a folder at `WORKSPACE_ROOT/some/relative/path` if it doesn't exist.
pub fn create_folder_relative_to_workspace(path: impl AsRef<Path>) -> Result<PathBuf> {
    let target_path = directory_relative_workspace_with_path_components(path);
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

impl<Items: Serialize + MaybeIsExpenses> DataFromDiskWithItemsOfKind<Items> {
    /// Returns the absolute path where the invoice will be saved.
    /// If the path is relative, it will be created relative to the workspace root.
    /// If the path is absolute, it will be returned as is.
    ///
    /// # Errors
    /// Returns an error if the path is relative and the folder cannot be created,
    /// or if the path is absolute but invalid.
    ///
    /// # Examples
    /// ```
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    /// let data = DataWithItemsPricedInSourceCurrency::builder()
    ///    .information(InvoiceInfoFull::sample())
    ///   .vendor(CompanyInformation::sample())
    ///   .client(CompanyInformation::sample())
    ///   .line_items(LineItemsPricedInSourceCurrency::sample())
    ///   .payment_info(PaymentInformation::sample())
    ///   .output_path(OutputPath::Name("invoice.pdf".into()))
    ///   .build();
    ///
    /// let absolute_path = data.absolute_path().unwrap();
    /// assert!(absolute_path.ends_with("invoice.pdf"));
    /// ```
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

impl DataWithItemsPricedInSourceCurrency {
    /// Converts the `DataWithItemsPricedInSourceCurrency` into a `DataTypstCompat`
    /// which is compatible with Typst rendering.
    /// This method prepares the invoice data for rendering by creating an
    /// `ExchangeRates` object and converting the line items into a flat structure.
    ///
    /// # Errors
    /// Returns an error if the line items cannot be converted to a flat structure.
    ///
    /// # Examples
    /// ```
    /// extern crate invoice_typst_logic;
    /// use invoice_typst_logic::prelude::*;
    /// let data = DataWithItemsPricedInSourceCurrency::sample();
    /// let exchange_rates = ExchangeRates::builder().rates(HashMap::from_iter([(Currency::GBP, UnitPrice::from(10.0)), (Currency::EUR, UnitPrice::from(8.0))])).target_currency(Currency::EUR).build();
    /// let result = data.to_typst(exchange_rates);
    /// assert!(result.is_ok(), "Expected conversion to succeed, got: {:?}", result);
    /// ```
    pub fn to_typst(self, exchange_rates: ExchangeRates) -> Result<DataTypstCompat> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use test_log::test;

    #[test]
    fn test_get_absolute_path_with_absolute_path() {
        let data = read_data_from_disk().unwrap();
        let path_buf = PathBuf::from("/absolute/path/to/invoice.pdf");
        let data = data
            .to_partial(
                ValidInput::builder()
                    .maybe_output_path(path_buf.clone())
                    .month(YearAndMonth::sample())
                    .build(),
            )
            .unwrap();
        assert_eq!(data.absolute_path().unwrap(), path_buf);
    }
}
