use crate::core::{definitions::GiftOrder, services::{DataFormatService, ExtractGiftOrders}};

pub struct TomlService{}

impl DataFormatService for TomlService{}

impl ExtractGiftOrders for TomlService {
    fn extract_gift_orders(&self, text: String) -> Vec<GiftOrder> {
        vec![]
    }
}
