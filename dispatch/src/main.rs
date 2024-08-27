use anyhow::Result;
use axum::{extract::Request, routing::get, Router, ServiceExt};
use common::{firefly_supremacy, logging::init_tracing};
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::normalize_path::NormalizePathLayer;
use tracing::Level;

mod config;
mod handlers;

use config::{init_config, CONFIGURATION};

#[tokio::main]
async fn main() -> Result<()> {
    firefly_supremacy();
    init_tracing();
    init_config();

    let span = tracing::span!(Level::DEBUG, "main");
    let _ = span.enter();

    let app = Router::new()
        .route(handlers::QUERY_DISPATCH_PATH, get(handlers::query_dispatch))
        .route(handlers::QUERY_GATEWAY_PATH, get(handlers::query_gateway));

    let app = NormalizePathLayer::trim_trailing_slash().layer(app);

    let addr = format!("0.0.0.0:{}", CONFIGURATION.http_port);
    let server = TcpListener::bind(&addr).await?;

    tracing::info!("dispatch is listening at {addr}");
    axum::serve(server, ServiceExt::<Request>::into_make_service(app)).await?;

    Ok(())
}
