mod core;
mod endpoints;
mod solutions;

use axum::{
    routing::{get, post},
    Router,
};
use solutions::{BasicMathService, JsonService, TomlService, YamlService};
use std::sync::Arc;

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let math_service = BasicMathService {};
    let toml_service = TomlService {};
    let json_service = JsonService {};
    let yaml_service = YamlService {};

    let state = AppState {
        json_service: Arc::new(json_service),
        math_service: Arc::new(math_service),
        toml_service: Arc::new(toml_service),
        yaml_service: Arc::new(yaml_service),
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
        .with_state(state);

    Ok(router.into())
}

#[derive(Clone)]
struct AppState {
    math_service: Arc<BasicMathService>,
    toml_service: Arc<TomlService>,
    json_service: Arc<JsonService>,
    yaml_service: Arc<YamlService>,
}

impl AppState {
    fn json_service(&self) -> &JsonService {
        &self.json_service
    }

    fn math_service(&self) -> &BasicMathService {
        &self.math_service
    }

    fn toml_service(&self) -> &TomlService {
        &self.toml_service
    }

    fn yaml_service(&self) -> &YamlService {
        &self.yaml_service
    }
}
