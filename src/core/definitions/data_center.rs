use std::{
    fmt::{Display, Formatter},
    net::Ipv4Addr,
};

pub struct Route {
    source: RouteSource,
    key: RouteKey,
    destination: RouteDestination,
}

pub struct RouteParameters {
    pub source: Ipv4Addr,
    pub fragment: RouteFragment,
}

pub enum RouteFragment {
    Key(Ipv4Addr),
    Destination(Ipv4Addr),
}

#[derive(Copy, Clone)]
pub struct RouteDestination(Ipv4Addr);

struct RouteKey(Ipv4Addr);

struct RouteSource(Ipv4Addr);

impl Route {
    pub fn destination(&self) -> RouteDestination {
        self.destination
    }

    pub fn new(parameters: RouteParameters) -> Route {
        let RouteParameters { source, fragment } = parameters;

        let source = RouteSource(source);

        let (key, destination) = match fragment {
            RouteFragment::Key(key) => {
                let key = RouteKey(key);
                let destination = source.destination(&key);

                (key, destination)
            }
            RouteFragment::Destination(destination) => {
                let destination = RouteDestination(destination);
                let key = source.key(&destination);

                (key, destination)
            }
        };

        Route {
            source,
            key,
            destination,
        }
    }
}

impl RouteSource {
    fn destination(&self, key: &RouteKey) -> RouteDestination {
        let address = add_addresses(self.0, key.0);

        RouteDestination(address)
    }

    fn key(&self, destination: &RouteDestination) -> RouteKey {
        todo!()
    }
}

impl Display for RouteDestination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn add_addresses(a: Ipv4Addr, b: Ipv4Addr) -> Ipv4Addr {
    let a = a.octets();
    let b = b.octets();

    Ipv4Addr::new(
        a[0].wrapping_add(b[0]),
        a[1].wrapping_add(b[1]),
        a[2].wrapping_add(b[2]),
        a[3].wrapping_add(b[3]),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_destination_math() {
        struct TestData {
            source: Ipv4Addr,
            key: Ipv4Addr,
            expected_destination: Ipv4Addr,
        }

        let test_data = vec![
            TestData {
                source: Ipv4Addr::new(10, 0, 0, 0),
                key: Ipv4Addr::new(1, 2, 3, 255),
                expected_destination: Ipv4Addr::new(11, 2, 3, 255),
            },
            TestData {
                source: Ipv4Addr::new(128, 128, 33, 0),
                key: Ipv4Addr::new(255, 0, 255, 33),
                expected_destination: Ipv4Addr::new(127, 128, 32, 33),
            },
        ];

        for data in test_data {
            let TestData {
                source,
                key,
                expected_destination,
            } = data;

            let source = RouteSource(source);
            let key = RouteKey(key);
            let destination = source.destination(&key);

            assert_eq!(
                destination.0, expected_destination,
                "{} + {}, expected: {}, got: {}",
                source.0, key.0, expected_destination, destination.0
            );
        }
    }
}
