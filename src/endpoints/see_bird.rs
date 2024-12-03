use axum::{
    body::Body,
    http::{header, Response, StatusCode},
    response::IntoResponse,
};

pub async fn see_bird() -> Response<Body> {
    (
        StatusCode::FOUND,
        [(
            header::LOCATION,
            "https://www.youtube.com/watch?v=9Gc4QTqslN4",
        )],
    )
        .into_response()
}
