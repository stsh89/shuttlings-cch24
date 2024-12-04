use crate::{
    core::{
        definitions::{RouteV6Destination, RouteV6Source},
        operations::{MissingV6KeyFragment, RestoreRouteOperation, RouteFragment},
    },
    endpoints::{EndpointError, EndpointResult},
    AppState,
};
use axum::extract::{Query, State};
use serde::Deserialize;
use std::net::Ipv6Addr;

#[derive(Deserialize)]
pub struct GetDestinationAddressQueryParameters {
    from: String,
    to: String,
}

pub async fn get_v6_route_key(
    query: Query<GetDestinationAddressQueryParameters>,
    State(state): State<AppState>,
) -> EndpointResult<String> {
    let GetDestinationAddressQueryParameters { from, to } = query.0;

    let source: Ipv6Addr = from.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: from. Expected valid IPv6 network address, got: {}",
            from
        ))
    })?;

    let destination: Ipv6Addr = to.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: to. Expected valid IPv6 network address, got: {}",
            to
        ))
    })?;

    let route = RestoreRouteOperation {
        math: state.math_service(),
    }
    .execute(RouteFragment::Missingv6Key(MissingV6KeyFragment {
        destination: RouteV6Destination::new(destination),
        source: RouteV6Source::new(source),
    }));

    let body = format!("{}", route.key());

    Ok(body)
}
