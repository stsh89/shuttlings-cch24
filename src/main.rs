mod core;
mod endpoints;
mod solutions;

use axum::{routing::get, Router};
use solutions::BasicMathService;
use std::sync::Arc;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let math_service = BasicMathService {};

    let state = AppState {
        math_service: Arc::new(math_service),
    };

    let router = Router::new()
        .route("/", get(endpoints::hello_bird))
        .route("/-1/seek", get(endpoints::see_bird))
        .route("/2/dest", get(endpoints::get_v4_route_destination))
        .route("/2/key", get(endpoints::get_v4_route_key))
        .route("/2/v6/dest", get(endpoints::get_v6_route_destination))
        .route("/2/v6/key", get(endpoints::get_v6_route_key))
        .with_state(state);

    Ok(router.into())
}

#[derive(Clone)]
struct AppState {
    math_service: Arc<BasicMathService>,
}

impl AppState {
    fn math_service(&self) -> &BasicMathService {
        &self.math_service
    }
}
