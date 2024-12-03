use crate::{
    core::operations::{CalculateRouteDestination, CalculateRouteDestinationParameters},
    endpoints::{EndpointError, EndpointResult},
};
use axum::extract::Query;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct GetDestinationAddressQueryParameters {
    from: String,
    key: String,
}

pub async fn get_destination_address(
    query: Query<GetDestinationAddressQueryParameters>,
) -> EndpointResult<String> {
    let GetDestinationAddressQueryParameters { from, key } = query.0;

    let source = from.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: from. Expected valid network address, got: {}",
            from
        ))
    })?;

    let key = key.parse().map_err(EndpointError::from).map_err(|err| {
        err.wrap_err(format!(
            "Incorrect query parameter: key. Expected valid network address, got: {}",
            key
        ))
    })?;

    let route =
        CalculateRouteDestination {}.execute(CalculateRouteDestinationParameters { source, key });

    let body = format!("{}", route.destination());

    Ok(body)
}
