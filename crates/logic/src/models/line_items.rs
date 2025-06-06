use crate::prelude::*;

/// Services or expenses included in this invoice to be paid by the client.
#[derive(Clone, Debug, Serialize, Deserialize, From)]
#[from(Vec<Item>, Item)]
pub enum LineItems {
    /// Service sold by the vendor to the client, e.g. `"App development"`
    Service(Item),
    /// Expense incurred by the vendor, travel expenses for a conference/summit/
    /// retreat
    Expenses(Vec<Item>),
}

impl From<LineItems> for LineItemsFlat {
    fn from(line_items: LineItems) -> Self {
        match line_items {
            LineItems::Service(service) => LineItemsFlat {
                service: Some(service),
                expenses: Vec::new(),
            },
            LineItems::Expenses(expenses) => LineItemsFlat {
                service: None,
                expenses,
            },
        }
    }
}

#[derive(Clone, Debug, Serialize, Getters)]
pub struct LineItemsFlat {
    /// Service sold by the vendor to the client, e.g. `"App development"`,
    /// might be `None` if no service is provided
    #[getset(get = "pub")]
    service: Option<Item>,

    /// Expense incurred by the vendor, travel expenses for a conference/summit/
    /// retreat,
    /// might be empty if no expenses are incurred
    #[getset(get = "pub")]
    expenses: Vec<Item>,
}
