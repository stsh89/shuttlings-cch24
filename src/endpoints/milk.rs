use super::EndpointResult;
use crate::{
    core::{definitions::Milk, operations::GetMilkOperation},
    AppState,
};
use axum::{
    extract::{Request, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use chrono::Utc;

pub async fn milk(State(state): State<AppState>, _req: Request) -> EndpointResult<Response> {
    println!("{}", Utc::now());

    let milk = GetMilkOperation {
        rate_limit_service: state.rate_limit_service(),
    }
    .execute();

    response(milk)
}

fn response(milk: Option<Milk>) -> EndpointResult<Response> {
    let Some(_milk) = milk else {
        return Ok((
            StatusCode::TOO_MANY_REQUESTS,
            "No milk available\n".to_string(),
        )
            .into_response());
    };

    Ok((StatusCode::OK, "Milk withdrawn\n".to_string()).into_response())
}
