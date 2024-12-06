use crate::core::{
    definitions::{Error, GiftOrder, GiftOrderParameters},
    services::GetManifest,
};
use std::num::NonZeroU32;

/// Ho ho ho! Santa's got his hands full this year with the grand Christmas
/// present delivery! He's all set to send out orders to his trusty warehouse at
/// the South Pole, but oh my, what a festive twist we have here!
///
/// You see, the magical undersea optic fiber cable Santa installed back in 2005
/// got a bit of a surprise - a ship dragged its anchor right through it! Now,
/// instead of sending messages through that secret snowy channel, Santa must
/// use the open Internet, where those mischievous hackers under the Grinch's
/// command are waiting to disrupt them.
///
/// But Santa, with his big belly laugh, says, "No time for encryption, my dear
/// elves! Christmas is just 20 days away, and we've got heaps of other magical
/// coding tasks to sprinkle with Christmas magic! Ho ho ho!"
///
/// Santa has a plan for evading the hackers' attempts at disrupting his gift
/// order requests: disguising them as Cargo.toml files!
///
/// Santa will be sending his gift orders as POST requests to /5/manifest with
/// TOML documents that are valid Cargo.toml manifests. The package.metadata
/// field allows arbitrary data to be stored in Cargo manifests, but for the
/// worker elves in the warehouse to understand the order, they need it written
/// out as a plain newline-separated list (with no newline at the end).
///
/// The package.metadata.orders field will be an array of maps that should have
/// item (String) and quantity (u32) fields. Invalid orders in the array should
/// be ignored. If the manifest does not contain any valid orders, the response
/// should be 204 No Content.
///
/// See [challenge page](https://console.shuttle.dev/shuttlings/cch24/challenge/5) for details.
pub struct ExtractGiftOrdersOperation<'a, T> {
    pub data_format_service: &'a T,
}

pub struct ExtractGiftOrdersParameters {
    pub text: String,
}

impl<'a, T> ExtractGiftOrdersOperation<'a, T>
where
    T: GetManifest,
{
    pub fn execute(
        &self,
        parameters: ExtractGiftOrdersParameters,
    ) -> Result<Vec<GiftOrder>, Error> {
        let ExtractGiftOrdersParameters { text } = parameters;

        let manifest = self.data_format_service.get_manifest(text)?;

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
