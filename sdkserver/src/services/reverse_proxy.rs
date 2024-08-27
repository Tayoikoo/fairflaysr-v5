use axum::{
    extract::{Request, State},
    http::uri::{PathAndQuery, Uri},
    response::{IntoResponse, Response},
};
use hyper::StatusCode;

use super::*;
use crate::config::CONFIGURATION;

const QUERY_DISPATCH: &str = "/query_dispatch";
const QUERY_GATEWAY: &str = "/query_gateway/:region_name";

pub fn setup_routes(router: Router<SdkContext>) -> Router<SdkContext> {
    router
        .route(QUERY_DISPATCH, get(forward_to_dispatch))
        .route(QUERY_GATEWAY, get(forward_to_dispatch))
}

async fn forward_to_dispatch(
    State(context): State<SdkContext>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map_or(path, PathAndQuery::as_str);

    let uri = format!("{}{}", CONFIGURATION.dispatch_endpoint, path_query);

    *req.uri_mut() = Uri::try_from(uri).unwrap();

    Ok(context
        .http_client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?
        .into_response())
}
