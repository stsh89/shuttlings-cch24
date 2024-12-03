mod failure;
mod get_destination_address;
mod hello_bird;
mod see_bird;

pub use get_destination_address::*;
pub use hello_bird::*;
pub use see_bird::*;

pub use failure::EndpointError;
pub type EndpointResult<T> = Result<T, EndpointError>;
