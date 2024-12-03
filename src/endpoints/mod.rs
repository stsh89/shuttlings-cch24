mod failure;
mod get_route_destination;
mod get_route_key;
mod hello_bird;
mod see_bird;

pub use get_route_destination::*;
pub use get_route_key::*;
pub use hello_bird::*;
pub use see_bird::*;

pub use failure::EndpointError;
pub type EndpointResult<T> = Result<T, EndpointError>;
