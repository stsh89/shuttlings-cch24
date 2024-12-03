use crate::core::definitions::{Route, RouteFragment, RouteParameters};
use std::net::Ipv4Addr;

#[cfg(doc)]
use crate::core::operations::CalculateRouteDestinationOperation;

/// Santa occasionally also wants to double check that the routing calculations are correct.
///
/// This operation is the reverse of [`CalculateRouteDestinationOperation`]
///
/// Expamples:
///
///    11.2.3.255 - 10.0.0.0 = 1.2.3.255
///    127.128.32.33 - 128.128.33.0 = 255.0.255.33
pub struct CalculateRouteKeyOperation {}

pub struct CalculateRouteKeyParameters {
    pub source: Ipv4Addr,
    pub destination: Ipv4Addr,
}

impl CalculateRouteKeyOperation {
    pub fn execute(&self, parameters: CalculateRouteKeyParameters) -> Route {
        let CalculateRouteKeyParameters {
            source,
            destination,
        } = parameters;

        Route::new(RouteParameters {
            source,
            fragment: RouteFragment::Destination(destination),
        })
    }
}
