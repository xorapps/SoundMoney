use borsh::{BorshDeserialize, BorshSerialize};

#[derive(
    Debug, Clone, Default, BorshDeserialize, BorshSerialize, PartialEq, PartialOrd, Eq, Ord,
)]
pub struct BillableItem {
    name: String,
    cost: String,
    img: String,
}

impl BillableItem {
    pub fn new() -> Self {
        BillableItem::default()
    }

    pub fn add_name(mut self, name: &str) -> Self {
        self.name = name.to_owned();

        self
    }

    pub fn add_cost(mut self, cost: &str) -> Self {
        self.cost = cost.to_owned();

        self
    }

    pub fn add_img(mut self, img: &str) -> Self {
        self.img = img.to_owned();

        self
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn cost(&self) -> &str {
        self.cost.as_str()
    }

    pub fn img(&self) -> &str {
        self.img.as_str()
    }
}
