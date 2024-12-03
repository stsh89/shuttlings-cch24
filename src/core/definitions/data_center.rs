use std::{
    fmt::{Display, Formatter},
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
};

pub enum Route {
    V4(RouteV4),
    V6(RouteV6),
}

pub enum RouteParameters {
    V4(RouteV4Parameters),
    V6(RouteV6Parameters),
}

pub struct RouteV4Parameters {
    pub destination: RouteV4Destination,
    pub key: RouteV4Key,
    pub source: RouteV4Source,
}

pub struct RouteV6Parameters {
    pub destination: RouteV6Destination,
    pub key: RouteV6Key,
    pub source: RouteV6Source,
}

pub struct RouteV4 {
    #[allow(dead_code)]
    source: RouteV4Source,

    key: RouteV4Key,
    destination: RouteV4Destination,
}

pub struct RouteV6 {
    #[allow(dead_code)]
    source: RouteV6Source,

    key: RouteV6Key,
    destination: RouteV6Destination,
}

#[derive(Copy, Clone)]
pub struct RouteDestination(IpAddr);

#[derive(Copy, Clone)]
pub struct RouteV4Destination(Ipv4Addr);

#[derive(Copy, Clone)]
pub struct RouteV6Destination(Ipv6Addr);

#[derive(Copy, Clone)]
pub struct RouteKey(IpAddr);

#[derive(Copy, Clone)]
pub struct RouteV4Key(Ipv4Addr);

#[derive(Copy, Clone)]
pub struct RouteV6Key(Ipv6Addr);

#[derive(Copy, Clone)]
pub struct RouteV4Source(Ipv4Addr);

#[derive(Copy, Clone)]
pub struct RouteV6Source(Ipv6Addr);

impl Route {
    pub fn destination(&self) -> RouteDestination {
        match self {
            Route::V4(route_v4) => RouteDestination(IpAddr::V4(route_v4.destination.0)),
            Route::V6(route_v6) => RouteDestination(IpAddr::V6(route_v6.destination.0)),
        }
    }

    pub fn key(&self) -> RouteKey {
        match self {
            Route::V4(route_v4) => RouteKey(IpAddr::V4(route_v4.key.0)),
            Route::V6(route_v6) => RouteKey(IpAddr::V6(route_v6.key.0)),
        }
    }

    pub fn new(parameters: RouteParameters) -> Self {
        match parameters {
            RouteParameters::V4(parameters) => Self::new_v4(parameters),
            RouteParameters::V6(parameters) => Self::new_v6(parameters),
        }
    }

    fn new_v4(parameters: RouteV4Parameters) -> Self {
        let RouteV4Parameters {
            destination,
            key,
            source,
        } = parameters;

        Self::V4(RouteV4 {
            source,
            key,
            destination,
        })
    }

    fn new_v6(parameters: RouteV6Parameters) -> Self {
        let RouteV6Parameters {
            destination,
            key,
            source,
        } = parameters;

        Self::V6(RouteV6 {
            source,
            key,
            destination,
        })
    }
}

impl RouteV4Destination {
    pub fn as_ipv4(&self) -> Ipv4Addr {
        self.0
    }

    pub fn new(destination: Ipv4Addr) -> Self {
        Self(destination)
    }
}

impl RouteV6Destination {
    pub fn as_ipv6(&self) -> Ipv6Addr {
        self.0
    }

    pub fn new(destination: Ipv6Addr) -> Self {
        Self(destination)
    }
}

impl RouteV4Key {
    pub fn as_ipv4(&self) -> Ipv4Addr {
        self.0
    }

    pub fn new(key: Ipv4Addr) -> Self {
        Self(key)
    }
}

impl RouteV6Key {
    pub fn as_ipv6(&self) -> Ipv6Addr {
        self.0
    }

    pub fn new(key: Ipv6Addr) -> Self {
        Self(key)
    }
}

impl RouteV4Source {
    pub fn as_ipv4(&self) -> Ipv4Addr {
        self.0
    }

    pub fn new(source: Ipv4Addr) -> Self {
        Self(source)
    }
}

impl RouteV6Source {
    pub fn as_ipv6(&self) -> Ipv6Addr {
        self.0
    }

    pub fn new(source: Ipv6Addr) -> Self {
        Self(source)
    }
}

impl Display for RouteDestination {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Display for RouteKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
