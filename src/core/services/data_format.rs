use crate::core::definitions::GiftOrder;

pub trait DataFormatService{}

pub trait ExtractGiftOrders: DataFormatService {
    fn extract_gift_orders(&self, text: String) -> Vec<GiftOrder>;
}
