use std::net::AddrParseError;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use eyre::Error;

#[derive(Debug, thiserror::Error)]
#[error("{report}")]
pub struct EndpointError {
    kind: EndpointErrorKind,
    report: Error,
}

#[derive(Debug)]
pub enum EndpointErrorKind {
    BadRequest,
}

impl EndpointError {
    pub fn bad_request(report: Error) -> Self {
        Self {
            kind: EndpointErrorKind::BadRequest,
            report,
        }
    }

    pub fn wrap_err(self, msg: String) -> Self {
        Self {
            report: self.report.wrap_err(msg),
            ..self
        }
    }
}

impl IntoResponse for EndpointError {
    fn into_response(self) -> Response {
        let Self { kind, report } = self;

        match kind {
            EndpointErrorKind::BadRequest => {
                (StatusCode::BAD_REQUEST, report.to_string()).into_response()
            }
        }
    }
}

impl From<AddrParseError> for EndpointError {
    fn from(value: AddrParseError) -> Self {
        Self::bad_request(Error::new(value))
    }
}
