use xor_image_handler::ImageWithMime;

use crate::Currency;

#[derive(Debug, Default)]
pub struct ReceiptService {
    logo: ImageWithMime,
    business_name: String,
    date_of_sale: String,
    tax: u32,
    items: Vec<(String, Currency)>,
}

impl ReceiptService {
    pub fn new() -> Self {
        ReceiptService::default()
    }

    pub fn add_logo(&mut self, logo: ImageWithMime) -> &mut Self {
        self.logo = logo;

        self
    }

    pub fn add_business_name(&mut self, business_name: &str) -> &mut Self {
        self.business_name = business_name.to_owned();

        self
    }

    pub fn add_date_of_sale(&mut self, date_of_sale: &str) -> &mut Self {
        self.date_of_sale = date_of_sale.to_owned();

        self
    }

    pub fn add_tax(&mut self, tax: u32) -> &mut Self {
        self.tax = tax;

        self
    }

    pub fn add_item(&mut self, item: (String, Currency)) -> &mut Self {
        self.items.push((item.0.to_owned(), item.1));

        self
    }
}
