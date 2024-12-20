mod failure;
mod get_v4_route_destination;
mod get_v4_route_key;
mod get_v6_route_destination;
mod get_v6_route_key;
mod gift_orders;
mod hello_bird;
mod milk;
mod see_bird;

pub use get_v4_route_destination::*;
pub use get_v4_route_key::*;
pub use get_v6_route_destination::*;
pub use get_v6_route_key::*;
pub use gift_orders::*;
pub use hello_bird::*;
pub use milk::*;
pub use see_bird::*;

pub use failure::EndpointError;
pub type EndpointResult<T> = Result<T, EndpointError>;
