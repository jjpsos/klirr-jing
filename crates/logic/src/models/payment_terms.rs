use crate::prelude::*;

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
