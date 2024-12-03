use crate::{
    core::{
        definitions::{RouteV4Key, RouteV4Source},
        operations::{MissingV4DestinationFragment, RestoreRouteOperation, RouteFragment},
    },
    endpoints::{EndpointError, EndpointResult},
    solutions::InMemoryNetwork,
};
use axum::extract::Query;
use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Deserialize)]
pub struct GetDestinationAddressQueryParameters {
    from: String,
    key: String,
}

pub async fn get_v4_route_destination(
    query: Query<GetDestinationAddressQueryParameters>,
) -> EndpointResult<String> {
    let GetDestinationAddressQueryParameters { from, key } = query.0;

    let source: Ipv4Addr = from.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: from. Expected valid IPv4 network address, got: {}",
            from
        ))
    })?;

    let key: Ipv4Addr = key.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: key. Expected valid IPv4 network address, got: {}",
            key
        ))
    })?;

    let route = RestoreRouteOperation {
        network_service: InMemoryNetwork {},
    }
    .execute(RouteFragment::MissingV4Destination(
        MissingV4DestinationFragment {
            key: RouteV4Key::new(key),
            source: RouteV4Source::new(source),
        },
    ));

    let body = format!("{}", route.destination());

    Ok(body)
}
