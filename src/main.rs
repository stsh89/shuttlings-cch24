mod core;
mod endpoints;
mod solutions;

use axum::{
    routing::{get, post},
    Router,
};
use solutions::{BasicMathService, JsonService, RateLimiter, TomlService, YamlService};
use std::sync::Arc;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let state = AppState {
        json_service: Arc::new(JsonService {}),
        math_service: Arc::new(BasicMathService {}),
        toml_service: Arc::new(TomlService {}),
        yaml_service: Arc::new(YamlService {}),
        rate_limit_service: Arc::new(RateLimiter::new()),
    };

    let router = Router::new()
        .route("/", get(endpoints::hello_bird))
        .route("/-1/seek", get(endpoints::see_bird))
        .route("/2/dest", get(endpoints::get_v4_route_destination))
        .route("/2/key", get(endpoints::get_v4_route_key))
        .route("/2/v6/dest", get(endpoints::get_v6_route_destination))
        .route("/2/v6/key", get(endpoints::get_v6_route_key))
        .route("/5/manifest", post(endpoints::gift_orders))
        .route("/9/milk", post(endpoints::milk))
        .route("/9/refill", post(endpoints::refill_milk))
        .with_state(state);

    Ok(router.into())
}

#[derive(Clone)]
struct AppState {
    math_service: Arc<BasicMathService>,
    toml_service: Arc<TomlService>,
    json_service: Arc<JsonService>,
    yaml_service: Arc<YamlService>,
    rate_limit_service: Arc<RateLimiter>,
}

impl AppState {
    fn json_service(&self) -> &JsonService {
        &self.json_service
    }

    fn math_service(&self) -> &BasicMathService {
        &self.math_service
    }

    fn rate_limit_service(&self) -> &RateLimiter {
        &self.rate_limit_service
    }

    fn toml_service(&self) -> &TomlService {
        &self.toml_service
    }

    fn yaml_service(&self) -> &YamlService {
        &self.yaml_service
    }

    fn reload_rate_limit_service(&mut self) {
        self.rate_limit_service = Arc::new(RateLimiter::new());
    }
}
