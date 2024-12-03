use crate::{
    core::{
        definitions::{RouteV4Destination, RouteV4Source},
        operations::{MissingV4KeyFragment, RestoreRouteOperation, RouteFragment},
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
    to: String,
}

pub async fn get_v4_route_key(
    query: Query<GetDestinationAddressQueryParameters>,
) -> EndpointResult<String> {
    let GetDestinationAddressQueryParameters { from, to } = query.0;

    let source: Ipv4Addr = from.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: from. Expected valid network address, got: {}",
            from
        ))
    })?;

    let destination: Ipv4Addr = to.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: to. Expected valid network address, got: {}",
            to
        ))
    })?;

    let route = RestoreRouteOperation {
        network_service: InMemoryNetwork {},
    }
    .execute(RouteFragment::Missingv4Key(MissingV4KeyFragment {
        destination: RouteV4Destination::new(destination),
        source: RouteV4Source::new(source),
    }));

    let body = format!("{}", route.key());

    Ok(body)
}
