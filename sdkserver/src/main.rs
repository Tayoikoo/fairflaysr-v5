use anyhow::Result;
use axum::body::Body;
use axum::extract::Request;
use axum::{Router, ServiceExt};
use common::firefly_supremacy;
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use mongodb::Client;
use services::{granter, mdk_shield, pages, reverse_proxy, takumi_api};
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use tracing::Level;

type HttpClient = hyper_util::client::legacy::Client<HttpConnector, Body>;

mod config;
mod database;
mod services;
mod util;

use common::logging::init_tracing;

use config::{init_config, CONFIGURATION};

#[derive(Clone)]
pub struct SdkContext {
    pub http_client: HttpClient,
    pub db_client: Client,
}

#[tokio::main]
async fn main() -> Result<()> {
    firefly_supremacy();
    init_tracing();
    init_config();

    let span = tracing::span!(Level::DEBUG, "main");
    let _ = span.enter();

    let app = create_router();

    let http_client: HttpClient =
        hyper_util::client::legacy::Client::<(), ()>::builder(TokioExecutor::new())
            .build(HttpConnector::new());

    let db_client = Client::with_uri_str(&CONFIGURATION.database.connection_string).await?;

    let app = app.with_state(SdkContext {
        http_client,
        db_client,
    });

    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    let addr = format!("0.0.0.0:{}", CONFIGURATION.http_port);
    let server = TcpListener::bind(&addr).await?;

    tracing::info!("sdkserver is listening at {addr}");
    axum::serve(server, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}

fn create_router() -> Router<SdkContext> {
    let mut router = Router::new();
    router = mdk_shield::setup_routes(router);
    router = takumi_api::setup_routes(router);
    router = granter::setup_routes(router);
    router = pages::register::setup_routes(router);
    reverse_proxy::setup_routes(router)
}
