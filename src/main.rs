mod core;
mod endpoints;

use axum::{routing::get, Router};

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(endpoints::hello_bird))
        .route("/-1/seek", get(endpoints::see_bird))
        .route("/2/dest", get(endpoints::get_destination_address));

    Ok(router.into())
}
