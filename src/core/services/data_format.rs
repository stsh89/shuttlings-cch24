use crate::core::definitions::{Error, GiftOrder};

pub trait DataFormatService {}

pub trait ExtractGiftOrders: DataFormatService {
    fn extract_gift_orders(&self, text: String) -> Result<Vec<GiftOrder>, Error>;
}
