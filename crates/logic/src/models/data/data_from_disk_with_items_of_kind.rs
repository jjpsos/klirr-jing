use std::fs;

pub const INVOICES_OUTPUT_DIR: &str = "invoices";

use crate::prelude::*;

pub type DataWithItemsPricedInSourceCurrency =
    DataFromDiskWithItemsOfKind<LineItemsPricedInSourceCurrency>;
pub type DataTypstCompat = DataFromDiskWithItemsOfKind<LineItemsFlat>;

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

impl<Items: Serialize + MaybeIsExpenses> DataFromDiskWithItemsOfKind<Items> {
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
