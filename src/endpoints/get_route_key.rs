use crate::{
    core::operations::{CalculateRouteKeyOperation, CalculateRouteKeyParameters},
    endpoints::{EndpointError, EndpointResult},
};
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetDestinationAddressQueryParameters {
    from: String,
    to: String,
}

pub async fn get_route_key(
    query: Query<GetDestinationAddressQueryParameters>,
) -> EndpointResult<String> {
    let GetDestinationAddressQueryParameters { from, to } = query.0;

    let source = from.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: from. Expected valid network address, got: {}",
            from
        ))
    })?;

    let destination = to.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: to. Expected valid network address, got: {}",
            to
        ))
    })?;

    let route = CalculateRouteKeyOperation {}.execute(CalculateRouteKeyParameters {
        source,
        destination,
    });

    let body = format!("{}", route.key());

    Ok(body)
}
