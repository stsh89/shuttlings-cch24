use std::num::NonZeroU32;

pub struct GiftOrder {
    item: String,
    quantity: NonZeroU32,
}

pub struct GiftOrderParameters {
    pub item: String,
    pub quantity: NonZeroU32,
}

impl GiftOrder {
    pub fn item(&self) -> &str {
        &self.item
    }

    pub fn new(parameters: GiftOrderParameters) -> Self {
        let GiftOrderParameters { item, quantity } = parameters;

        GiftOrder { item, quantity }
    }

    pub fn quantity(&self) -> NonZeroU32 {
        self.quantity
    }
}
