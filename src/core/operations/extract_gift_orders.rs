use crate::core::{definitions::GiftOrder, services::ExtractGiftOrders};

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
    T: ExtractGiftOrders,
{
    pub fn execute(&self, parameters: ExtractGiftOrdersParameters) -> Vec<GiftOrder> {
        let ExtractGiftOrdersParameters { text } = parameters;

        self.data_format_service.extract_gift_orders(text)
    }
}
