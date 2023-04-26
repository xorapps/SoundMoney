use crate::BillableItem;
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(Debug, Clone, Default, BorshDeserialize, BorshSerialize, PartialEq, PartialOrd)]
pub struct CpiDetails {
    customer_name: String,
    date_of_issue: String,
    items: Vec<BillableItem>,
}

impl CpiDetails {
    pub fn new() -> Self {
        CpiDetails::default()
    }

    pub fn add_name(mut self, name: String) -> Self {
        self.customer_name = name;

        self
    }

    pub fn add_date(mut self, date_of_issue: String) -> Self {
        self.date_of_issue = date_of_issue;

        self
    }

    pub fn add_items(mut self, item: BillableItem) -> Self {
        self.items.push(item);

        self
    }

    pub fn customer_name(&self) -> &str {
        self.customer_name.as_str()
    }

    pub fn date_of_issue(&self) -> &str {
        self.date_of_issue.as_str()
    }

    pub fn items(&self) -> &[BillableItem] {
        self.items.as_slice()
    }
}
