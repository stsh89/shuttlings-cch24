use super::EndpointResult;
use crate::{
    core::{
        definitions::{Error as CoreError, GiftOrder},
        operations::{ExtractGiftOrdersOperation, ExtractGiftOrdersParameters},
    },
    endpoints::EndpointError,
    AppState,
};
use axum::{
    extract::{Request, State},
    http::header::CONTENT_TYPE,
    RequestExt,
};
use eyre::{eyre, Report};

pub async fn gift_orders(State(state): State<AppState>, req: Request) -> EndpointResult<String> {
    let content_type = parse_content_type(&req)?;
    let text = parse_body(req).await?;

    let response_body = execute_operation(OperationParameters {
        state,
        content_type,
        text,
    })?;

    Ok(response_body)
}

struct OperationParameters {
    state: AppState,
    content_type: ContentType,
    text: String,
}

struct ExtractGiftOrdersFromTextParameters {
    state: AppState,
    text: String,
}

enum ContentType {
    Json,
    Toml,
    Yaml,
}

fn execute_operation(parameters: OperationParameters) -> EndpointResult<String> {
    let OperationParameters {
        content_type,
        state,
        text,
    } = parameters;

    let gift_orders = match content_type {
        ContentType::Json => {
            extract_gift_orders_from_json(ExtractGiftOrdersFromTextParameters { state, text })
        }
        ContentType::Toml => {
            extract_gift_orders_from_toml(ExtractGiftOrdersFromTextParameters { state, text })
        }
        ContentType::Yaml => {
            extract_gift_orders_from_yaml(ExtractGiftOrdersFromTextParameters { state, text })
        }
    }?;

    if gift_orders.is_empty() {
        return Err(EndpointError::no_content());
    }

    let body = gift_orders
        .into_iter()
        .map(|gf| format!("{}: {}", gf.item(), gf.quantity()))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(body)
}

fn extract_gift_orders_from_toml(
    parameters: ExtractGiftOrdersFromTextParameters,
) -> EndpointResult<Vec<GiftOrder>> {
    let ExtractGiftOrdersFromTextParameters { state, text } = parameters;

    let gift_ordes = ExtractGiftOrdersOperation {
        data_format_service: state.toml_service(),
    }
    .execute(ExtractGiftOrdersParameters { text })
    .map_err(from_core_error)?;

    Ok(gift_ordes)
}

fn extract_gift_orders_from_json(
    parameters: ExtractGiftOrdersFromTextParameters,
) -> EndpointResult<Vec<GiftOrder>> {
    let ExtractGiftOrdersFromTextParameters { state, text } = parameters;

    let gift_ordes = ExtractGiftOrdersOperation {
        data_format_service: state.json_service(),
    }
    .execute(ExtractGiftOrdersParameters { text })
    .map_err(from_core_error)?;

    Ok(gift_ordes)
}

fn extract_gift_orders_from_yaml(
    parameters: ExtractGiftOrdersFromTextParameters,
) -> EndpointResult<Vec<GiftOrder>> {
    let ExtractGiftOrdersFromTextParameters { state, text } = parameters;

    let gift_ordes = ExtractGiftOrdersOperation {
        data_format_service: state.yaml_service(),
    }
    .execute(ExtractGiftOrdersParameters { text })
    .map_err(from_core_error)?;

    Ok(gift_ordes)
}

fn from_core_error(err: CoreError) -> EndpointError {
    if err.is_corrupted_data_format() {
        return EndpointError::bad_request(err.report().wrap_err("Invalid manifest"));
    }

    if err.is_missing_keyword() {
        return EndpointError::bad_request(err.report().wrap_err("Magic keyword not provided"));
    }

    EndpointError::internal(err.report())
}

async fn parse_body(req: Request) -> EndpointResult<String> {
    req.extract()
        .await
        .map_err(Report::new)
        .map_err(EndpointError::internal)
}

fn parse_content_type(req: &Request) -> EndpointResult<ContentType> {
    let content_type_header = req.headers().get(CONTENT_TYPE).cloned();

    let Some(content_type_header) = content_type_header else {
        return Err(EndpointError::unsupported_media_type(Report::msg(
            "Missing media type",
        )));
    };

    let content_type = content_type_header
        .to_str()
        .map_err(Report::new)
        .map_err(EndpointError::internal)?;

    if content_type.starts_with("application/json") {
        return Ok(ContentType::Json);
    }

    if content_type.starts_with("application/toml") {
        return Ok(ContentType::Toml);
    }

    if content_type.starts_with("application/yaml") {
        return Ok(ContentType::Yaml);
    }

    Err(EndpointError::unsupported_media_type(eyre!(
        "Unsupported media type. Expected: application/toml, got: {}",
        content_type
    )))
}
