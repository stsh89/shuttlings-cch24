use crate::{
    core::{
        definitions::{RouteV6Key, RouteV6Source},
        operations::{MissingV6DestinationFragment, RestoreRouteOperation, RouteFragment},
    },
    endpoints::{EndpointError, EndpointResult},
    solutions::InMemoryNetwork,
};
use axum::extract::Query;
use serde::Deserialize;
use std::net::Ipv6Addr;

#[derive(Deserialize)]
pub struct GetDestinationAddressQueryParameters {
    from: String,
    key: String,
}

pub async fn get_v6_route_destination(
    query: Query<GetDestinationAddressQueryParameters>,
) -> EndpointResult<String> {
    let GetDestinationAddressQueryParameters { from, key } = query.0;

    let source: Ipv6Addr = from.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: from. Expected valid IPv6network address, got: {}",
            from
        ))
    })?;

    let key: Ipv6Addr = key.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: key. Expected valid IPv6 network address, got: {}",
            key
        ))
    })?;

    let route = RestoreRouteOperation {
        network_service: InMemoryNetwork {},
    }
    .execute(RouteFragment::MissingV6Destination(
        MissingV6DestinationFragment {
            key: RouteV6Key::new(key),
            source: RouteV6Source::new(source),
        },
    ));

    let body = format!("{}", route.destination());

    Ok(body)
}
