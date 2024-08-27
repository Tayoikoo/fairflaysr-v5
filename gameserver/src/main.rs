use anyhow::Result;

mod config;
mod database;
mod game;
mod net;
mod util;

use common::data::init_assets;
use common::firefly_supremacy;
use common::logging::init_tracing;
use config::{init_config, CONFIGURATION};

#[tokio::main]
async fn main() -> Result<()> {
    firefly_supremacy();
    init_tracing();
    init_config();
    init_assets();

    database::init().await?;
    net::gateway::listen("0.0.0.0", CONFIGURATION.tcp_port).await
}
