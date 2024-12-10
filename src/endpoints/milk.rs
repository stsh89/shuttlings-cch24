use axum::extract::{Request, State};

use crate::AppState;

use super::EndpointResult;

pub async fn milk(State(_state): State<AppState>, _req: Request) -> EndpointResult<String> {
    Ok("Milk withdrawn\n".to_string())
}
