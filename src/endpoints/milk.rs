use super::EndpointResult;
use crate::{
    core::{
        definitions::{Milk, MilkVolume},
        operations::GetMilkOperation,
    },
    endpoints::EndpointError,
    AppState,
};
use axum::{
    extract::{Request, State},
    http::{header::CONTENT_TYPE, StatusCode},
    response::{IntoResponse, Response},
    RequestExt,
};
use chrono::Utc;
use eyre::Report;
use serde::Deserialize;

enum RequestKind {
    Withdrawal,
    UnitConversion { from: Unit, to: Unit, value: f32 },
}

#[derive(Deserialize)]
enum UnitConversionRequest {
    #[serde(rename = "liters")]
    LitersToGallons(f32),

    #[serde(rename = "gallons")]
    GallonsToLiters(f32),

    #[serde(rename = "litres")]
    LitersToPints(f32),

    #[serde(rename = "pints")]
    PintsToLiters(f32),
}

enum Unit {
    Liters,
    Gallons,
    Pints,
}

enum ContentType {
    Json,
}

pub async fn milk(State(state): State<AppState>, req: Request) -> EndpointResult<Response> {
    println!("{}", Utc::now());

    GetMilkOperation {
        rate_limit_service: state.rate_limit_service(),
    }
    .execute()
    .map_err(|err| {
        if err.is_rate_limited() {
            return EndpointError::too_many_requests(Report::msg("No milk available\n"));
        };

        EndpointError::internal(Report::new(err))
    })?;

    let request_kind = RequestKind::try_from_request(req).await?;

    let body = match request_kind {
        RequestKind::Withdrawal => "Milk withdrawn\n".to_string(),
        RequestKind::UnitConversion { to, from, value } => {
            let volumen = match from {
                Unit::Liters => MilkVolume::Liters(value),
                Unit::Gallons => MilkVolume::Gallons(value),
                Unit::Pints => MilkVolume::Pints(value),
            };

            let milk = Milk::new(volumen);

            match to {
                Unit::Liters => serde_json::json!({"liters": milk.liters()}).to_string(),
                Unit::Gallons => serde_json::json!({"gallons": milk.gallons()}).to_string(),
                Unit::Pints => serde_json::json!({"pints": milk.pints()}).to_string(),
            }
        }
    };

    Ok((StatusCode::OK, body).into_response())
}

impl RequestKind {
    async fn try_from_request(req: Request) -> EndpointResult<RequestKind> {
        let Some(content_type) = parse_content_type(&req) else {
            return Ok(RequestKind::Withdrawal);
        };

        match content_type {
            ContentType::Json => {
                let body = parse_body(req).await?;
                let request: UnitConversionRequest = serde_json::from_str(&body)
                    .map_err(Report::new)
                    .map_err(|err| err.wrap_err(""))
                    .map_err(EndpointError::bad_request)?;

                match request {
                    UnitConversionRequest::LitersToGallons(value) => {
                        Ok(RequestKind::UnitConversion {
                            from: Unit::Liters,
                            to: Unit::Gallons,
                            value,
                        })
                    }
                    UnitConversionRequest::GallonsToLiters(value) => {
                        Ok(RequestKind::UnitConversion {
                            from: Unit::Gallons,
                            to: Unit::Liters,
                            value,
                        })
                    }
                    UnitConversionRequest::LitersToPints(value) => {
                        Ok(RequestKind::UnitConversion {
                            from: Unit::Liters,
                            to: Unit::Pints,
                            value,
                        })
                    }
                    UnitConversionRequest::PintsToLiters(value) => {
                        Ok(RequestKind::UnitConversion {
                            from: Unit::Pints,
                            to: Unit::Liters,
                            value,
                        })
                    }
                }
            }
        }
    }
}

async fn parse_body(req: Request) -> EndpointResult<String> {
    req.extract()
        .await
        .map_err(Report::new)
        .map_err(EndpointError::internal)
}

fn parse_content_type(req: &Request) -> Option<ContentType> {
    let content_type_header = req.headers().get(CONTENT_TYPE).cloned()?;

    let content_type = content_type_header.to_str().ok()?;

    if content_type.starts_with("application/json") {
        return Some(ContentType::Json);
    }

    None
}
