use crate::prelude::*;

#[derive(Clone, Debug, Serialize, Deserialize, Getters)]
pub struct ExpensedMonths {
    explanation: String,
    #[getset(get = "pub")]
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
