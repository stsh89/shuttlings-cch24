use crate::core::definitions::{Route, RouteFragment, RouteParameters};
use std::net::Ipv4Addr;

/// # Operation description
///
/// Santa is having his network routers in the data center igloo upgraded to the
/// next generation. For reasons unknown, he uses a special IP routing algorithm
/// to obfuscate the traffic on the internal network. (An elf said that it just
/// looks like a terrible implementation of symmetric encryption.) He now needs
/// your help to implement a simple web API for verifying the calculations in the
/// routing algorithm.
///
/// The algorithm for IPv4 adresses is as follows:
///
/// To calculate the destination IP of a packet, take the source IP and apply a
/// key address. The formula from + key == dest (where "+" is overflowing
/// addition) is applied to each of the four octets separately.
///
/// Tips:
///
/// Overflowing addition for the u8 type means that adding 255 and 1 gives 0 (the
/// values wrap around in the interval 0-255).
///
/// Examples:
///
///     10.0.0.0 + 1.2.3.255 = 11.2.3.255
///     128.128.33.0 + 255.0.255.33 = 127.128.32.33
///
/// See [challenge page](https://console.shuttle.dev/shuttlings/cch24/challenge/2) for details.
pub struct CalculateRouteDestination {}

pub struct CalculateRouteDestinationParameters {
    pub source: Ipv4Addr,
    pub key: Ipv4Addr,
}

impl CalculateRouteDestination {
    pub fn execute(&self, parameters: CalculateRouteDestinationParameters) -> Route {
        let CalculateRouteDestinationParameters { source, key } = parameters;

        Route::new(RouteParameters {
            source,
            fragment: RouteFragment::Key(key),
        })
    }
}
