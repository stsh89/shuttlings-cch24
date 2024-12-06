use crate::core::{
    definitions::{Error, GiftOrder, GiftOrderParameters},
    services::{DataFormatService, ExtractGiftOrders},
};
use cargo_manifest::Manifest;
use eyre::Report;
use std::{num::NonZeroU32, str::FromStr};

pub struct TomlService {}

impl DataFormatService for TomlService {}

impl ExtractGiftOrders for TomlService {
    fn extract_gift_orders(&self, text: String) -> Result<Vec<GiftOrder>, Error> {
        let manifest = Manifest::from_str(&text)
            .map_err(Report::new)
            .map_err(Error::corrupted_data_format)?;

        let Some(package) = manifest.package else {
            return Ok(vec![]);
        };

        let Some(keywords) = package.keywords.and_then(|v| v.as_local()) else {
            return Err(Error::missing_keyword());
        };

        if !keywords.contains(&"Christmas 2024".to_string()) {
            return Err(Error::missing_keyword());
        }

        let Some(metadata) = package.metadata else {
            return Ok(vec![]);
        };

        let Some(table) = metadata.as_table() else {
            return Ok(vec![]);
        };

        let Some(gift_orders) = table.get("orders").and_then(|v| v.as_array()) else {
            return Ok(vec![]);
        };

        let gift_orders = gift_orders.iter().filter_map(maybe_gift_order).collect();

        Ok(gift_orders)
    }
}

fn maybe_gift_order(value: &toml::Value) -> Option<GiftOrder> {
    let table = value.as_table()?;

    let item = table.get("item")?.as_str().map(|s| s.to_string())?;
    let quantity = table
        .get("quantity")?
        .as_integer()
        .and_then(|v| v.try_into().ok())
        .and_then(NonZeroU32::new)?;

    Some(GiftOrder::new(GiftOrderParameters { item, quantity }))
}
