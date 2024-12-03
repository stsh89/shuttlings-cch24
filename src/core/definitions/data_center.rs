use std::{
    fmt::{Display, Formatter},
    net::{IpAddr, Ipv4Addr},
};

pub enum Route {
    V4(RouteV4),
}

pub enum RouteParameters {
    V4(RouteV4Parameters),
}

pub struct RouteV4Parameters {
    pub destination: RouteV4Destination,
    pub key: RouteV4Key,
    pub source: RouteV4Source,
}

pub struct RouteV4 {
    #[allow(dead_code)]
    source: RouteV4Source,

    key: RouteV4Key,
    destination: RouteV4Destination,
}

#[derive(Copy, Clone)]
pub struct RouteDestination(IpAddr);

#[derive(Copy, Clone)]
pub struct RouteV4Destination(Ipv4Addr);

#[derive(Copy, Clone)]
pub struct RouteKey(IpAddr);

#[derive(Copy, Clone)]
pub struct RouteV4Key(Ipv4Addr);

pub struct RouteV4Source(Ipv4Addr);

impl Route {
    pub fn destination(&self) -> RouteDestination {
        match self {
            Route::V4(route_v4) => RouteDestination(IpAddr::V4(route_v4.destination.0)),
        }
    }

    pub fn key(&self) -> RouteKey {
        match self {
            Route::V4(route_v4) => RouteKey(IpAddr::V4(route_v4.key.0)),
        }
    }

    pub fn new(parameters: RouteParameters) -> Self {
        match parameters {
            RouteParameters::V4(parameters) => Self::new_v4(parameters),
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
}

impl RouteV4Destination {
    pub fn as_ipv4(&self) -> Ipv4Addr {
        self.0
    }

    pub fn new(destination: Ipv4Addr) -> Self {
        Self(destination)
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

impl RouteV4Key {
    pub fn as_ipv4(&self) -> Ipv4Addr {
        self.0
    }

    pub fn new(key: Ipv4Addr) -> Self {
        Self(key)
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
