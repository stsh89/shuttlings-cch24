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
    UnsupportedMediaType,
    NoContent,
    Internal,
}

impl EndpointError {
    pub fn bad_request(report: Error) -> Self {
        Self {
            kind: EndpointErrorKind::BadRequest,
            report,
        }
    }

    pub fn internal(report: Error) -> Self {
        Self {
            kind: EndpointErrorKind::Internal,
            report,
        }
    }

    pub fn no_content() -> Self {
        Self {
            kind: EndpointErrorKind::NoContent,
            report: Error::msg(""),
        }
    }

    pub fn unsupported_media_type(report: Error) -> Self {
        Self {
            kind: EndpointErrorKind::UnsupportedMediaType,
            report
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
        use EndpointErrorKind as Kind;

        let Self { kind, report } = self;

        match kind {
            Kind::BadRequest => (StatusCode::BAD_REQUEST, report.to_string()).into_response(),
            Kind::UnsupportedMediaType => StatusCode::UNSUPPORTED_MEDIA_TYPE.into_response(),
            Kind::NoContent => StatusCode::NO_CONTENT.into_response(),
            Kind::Internal => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

impl From<AddrParseError> for EndpointError {
    fn from(value: AddrParseError) -> Self {
        Self::bad_request(Error::new(value))
    }
}
