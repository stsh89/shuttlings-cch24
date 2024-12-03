mod internals;

use crate::core::{
    definitions::{Route, RouteParameters, RouteV4Destination, RouteV4Key, RouteV4Parameters},
    operations::{MissingV4DestinationFragment, MissingV4KeyFragment, RouteFragment},
    services::{NetworkService, RestoreRoute},
};

pub struct InMemoryNetwork;

impl NetworkService for InMemoryNetwork {}

impl RestoreRoute for InMemoryNetwork {
    fn restore_route(&self, fragment: RouteFragment) -> Route {
        match fragment {
            RouteFragment::MissingV4Destination(fragment) => {
                restore_when_missing_v4_destination(fragment)
            }
            RouteFragment::Missingv4Key(fragment) => restore_when_missing_v4_key(fragment),
        }
    }
}

fn restore_when_missing_v4_destination(fragment: MissingV4DestinationFragment) -> Route {
    let MissingV4DestinationFragment { key, source } = fragment;

    let destination = internals::add_v4_addresses(source.as_ipv4(), key.as_ipv4());
    let destination = RouteV4Destination::new(destination);

    Route::new(RouteParameters::V4(RouteV4Parameters {
        destination,
        key,
        source,
    }))
}

fn restore_when_missing_v4_key(fragment: MissingV4KeyFragment) -> Route {
    let MissingV4KeyFragment {
        destination,
        source,
    } = fragment;

    let key = internals::sub_v4_address(destination.as_ipv4(), source.as_ipv4());
    let key = RouteV4Key::new(key);

    Route::new(RouteParameters::V4(RouteV4Parameters {
        key,
        destination,
        source,
    }))
}
