use super::EndpointResult;
use crate::{
    core::{definitions::Milk, operations::GetMilkOperation},
    AppState,
};
use axum::extract::{Request, State};
use chrono::Utc;

pub async fn milk(State(state): State<AppState>, _req: Request) -> EndpointResult<String> {
    println!("{}", Utc::now());

    let milk = GetMilkOperation {
        rate_limit_service: state.rate_limit_service(),
    }
    .execute();

    let body = body(milk);

    Ok(body)
}

fn body(milk: Option<Milk>) -> String {
    let Some(_milk) = milk else {
        return "No milk available\n".to_string();
    };

    "Milk withdrawn\n".to_string()
}
