use crate::core::{
    definitions::{GiftOrder, GiftOrderParameters},
    services::{DataFormatService, ExtractGiftOrders},
};
use std::num::NonZeroU32;
use toml::Table;

pub struct TomlService {}

impl DataFormatService for TomlService {}

impl ExtractGiftOrders for TomlService {
    fn extract_gift_orders(&self, text: String) -> Vec<GiftOrder> {
        let Some(table): Option<Table> = text.parse().ok() else {
            return vec![];
        };

        let Some(gift_orders) = table
            .get("package")
            .and_then(|v| v.as_table())
            .and_then(|v| v.get("metadata"))
            .and_then(|v| v.as_table())
            .and_then(|v| v.get("orders"))
            .and_then(|v| v.as_array())
        else {
            return vec![];
        };

        gift_orders
            .into_iter()
            .map(maybe_gift_order)
            .filter_map(|gift_order| gift_order)
            .collect()
    }
}

fn maybe_gift_order(value: &toml::Value) -> Option<GiftOrder> {
    let Some(table) = value.as_table() else {
        return None;
    };

    let Some(item) = table.get("item")?.as_str().map(|s| s.to_string()) else {
        return None;
    };

    let Some(quantity) = table
        .get("quantity")?
        .as_integer()
        .and_then(|v| v.try_into().ok())
        .and_then(NonZeroU32::new)
    else {
        return None;
    };

    Some(GiftOrder::new(GiftOrderParameters { item, quantity }))
}
