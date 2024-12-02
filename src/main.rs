use axum::{
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

async fn hello_bird() -> &'static str {
    "Hello, bird!"
}

async fn see_bird() -> Response {
    (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )],
    )
        .into_response()
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new()
        .route("/", get(hello_bird))
        .route("/-1/seek", get(see_bird));

    Ok(router.into())
}
