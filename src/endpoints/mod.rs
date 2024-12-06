mod failure;
mod get_v4_route_destination;
mod get_v4_route_key;
mod get_v6_route_destination;
mod get_v6_route_key;
mod hello_bird;
mod see_bird;
mod gift_orders;

pub use get_v4_route_destination::*;
pub use get_v4_route_key::*;
pub use get_v6_route_destination::*;
pub use get_v6_route_key::*;
pub use hello_bird::*;
pub use see_bird::*;
pub use gift_orders::*;

pub use failure::EndpointError;
pub type EndpointResult<T> = Result<T, EndpointError>;
