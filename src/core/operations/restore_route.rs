use crate::core::{
    definitions::{
        Route, RouteParameters, RouteV4Destination, RouteV4Key, RouteV4Parameters, RouteV4Source,
        RouteV6Destination, RouteV6Key, RouteV6Parameters, RouteV6Source,
    },
    services::{AddWithOverflow, MathService, SubWithOverflow, Xor},
};

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
///
/// Santa occasionally also wants to double check that the routing calculations are correct (reverses the calculation).
///
/// Expamples:
///
///    11.2.3.255 - 10.0.0.0 = 1.2.3.255
///    127.128.32.33 - 128.128.33.0 = 255.0.255.33
///
/// To keep up with the times, Santa also wants to use this type of routing for
/// IPv6 packets. He became a bit bored with elementary school math and decided
/// that for IPv6 packets, the algorithm should use XOR instead of overflowing
/// addition.
///
/// See [challenge page](https://console.shuttle.dev/shuttlings/cch24/challenge/2) for details.
pub struct RestoreRouteOperation<'a, T>
where
    T: MathService,
{
    pub math: &'a T,
}

pub enum RouteFragment {
    MissingV4Destination(MissingV4DestinationFragment),
    Missingv4Key(MissingV4KeyFragment),
    MissingV6Destination(MissingV6DestinationFragment),
    Missingv6Key(MissingV6KeyFragment),
}

pub struct MissingV4DestinationFragment {
    pub key: RouteV4Key,
    pub source: RouteV4Source,
}

pub struct MissingV6DestinationFragment {
    pub key: RouteV6Key,
    pub source: RouteV6Source,
}

pub struct MissingV4KeyFragment {
    pub destination: RouteV4Destination,
    pub source: RouteV4Source,
}

pub struct MissingV6KeyFragment {
    pub destination: RouteV6Destination,
    pub source: RouteV6Source,
}

impl<'a, T> RestoreRouteOperation<'a, T>
where
    T: AddWithOverflow + SubWithOverflow + Xor,
{
    pub fn execute(&self, fragment: RouteFragment) -> Route {
        use RouteFragment as RF;

        match fragment {
            RF::MissingV4Destination(fragment) => self.missing_v4_destination(fragment),
            RF::Missingv4Key(fragment) => self.missing_v4_key(fragment),
            RF::MissingV6Destination(fragment) => self.missing_v6_destination(fragment),
            RF::Missingv6Key(fragment) => self.missing_v6_key(fragment),
        }
    }

    fn calculate_v4_key(
        &self,
        source: RouteV4Source,
        destination: RouteV4Destination,
    ) -> RouteV4Key {
        let octets: [u8; 4] = destination
            .octets()
            .into_iter()
            .zip(source.octets())
            .map(|(a, b)| self.math.sub_with_overflow(a, b))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        RouteV4Key::new(octets.into())
    }

    fn calculate_v4_destination(
        &self,
        source: RouteV4Source,
        key: RouteV4Key,
    ) -> RouteV4Destination {
        let octets: [u8; 4] = source
            .octets()
            .into_iter()
            .zip(key.octets())
            .map(|(a, b)| self.math.add_with_overflow(a, b))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        RouteV4Destination::new(octets.into())
    }

    fn calculate_v6_key(
        &self,
        source: RouteV6Source,
        destination: RouteV6Destination,
    ) -> RouteV6Key {
        let octets: [u8; 16] = destination
            .octets()
            .into_iter()
            .zip(source.octets())
            .map(|(a, b)| self.math.xor(a, b))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        RouteV6Key::new(octets.into())
    }

    fn calculate_v6_destination(
        &self,
        source: RouteV6Source,
        key: RouteV6Key,
    ) -> RouteV6Destination {
        let octets: [u8; 16] = source
            .octets()
            .into_iter()
            .zip(key.octets())
            .map(|(a, b)| self.math.xor(a, b))
            .collect::<Vec<u8>>()
            .try_into()
            .unwrap();

        RouteV6Destination::new(octets.into())
    }

    fn missing_v4_destination(&self, fragment: MissingV4DestinationFragment) -> Route {
        let MissingV4DestinationFragment { key, source } = fragment;

        let destination = self.calculate_v4_destination(source, key);

        Route::new(RouteParameters::V4(RouteV4Parameters {
            destination,
            key,
            source,
        }))
    }

    fn missing_v6_destination(&self, fragment: MissingV6DestinationFragment) -> Route {
        let MissingV6DestinationFragment { key, source } = fragment;

        let destination = self.calculate_v6_destination(source, key);

        Route::new(RouteParameters::V6(RouteV6Parameters {
            destination,
            key,
            source,
        }))
    }

    fn missing_v4_key(&self, fragment: MissingV4KeyFragment) -> Route {
        let MissingV4KeyFragment {
            source,
            destination,
        } = fragment;

        let key = self.calculate_v4_key(source, destination);

        Route::new(RouteParameters::V4(RouteV4Parameters {
            destination,
            key,
            source,
        }))
    }

    fn missing_v6_key(&self, fragment: MissingV6KeyFragment) -> Route {
        let MissingV6KeyFragment {
            source,
            destination,
        } = fragment;

        let key = self.calculate_v6_key(source, destination);

        Route::new(RouteParameters::V6(RouteV6Parameters {
            destination,
            key,
            source,
        }))
    }
}
