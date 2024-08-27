use common::{config::DatabaseConfig, util::load_or_create_config};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::from_str;

const DEFAULT_CONFIG: &str = include_str!("../sdkserver.json");

pub fn init_config() {
    let _configuration = &*CONFIGURATION;
}

#[derive(Deserialize)]
pub struct SDKServerConfiguration {
    pub http_port: u16,
    pub dispatch_endpoint: String,
    pub database: DatabaseConfig,
    pub disable_password_check: bool,
}

lazy_static! {
    pub static ref CONFIGURATION: SDKServerConfiguration = {
        let data = load_or_create_config("sdkserver.json", DEFAULT_CONFIG);
        from_str(&data).unwrap()
    };
}
