use std::collections::HashMap;

use common::util::load_or_create_config;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::from_str;

const DEFAULT_CONFIG: &str = include_str!("../dispatch.json");

pub fn init_config() {
    let _configuration = &*CONFIGURATION;
}

#[derive(Deserialize)]
pub struct DispatchServerConfiguration {
    pub http_port: u16,
    pub game_servers: HashMap<String, GameServerConfig>,
    pub versions: HashMap<String, VersionConfig>,
}

#[derive(Deserialize)]
pub struct VersionConfig {
    pub asset_bundle_url: String,
    pub ex_resource_url: String,
    pub lua_url: String,
    pub lua_version: String,
}

#[derive(Deserialize)]
pub struct GameServerConfig {
    pub name: String,
    pub title: String,
    pub dispatch_url: String,
    pub env_type: String,
    pub gateserver_ip: String,
    pub gateserver_port: u16,
    pub gateserver_protocol: GatewayProtocolType,
}

#[derive(Deserialize, Eq, PartialEq)]
pub enum GatewayProtocolType {
    Tcp,
    Kcp,
}

lazy_static! {
    pub static ref CONFIGURATION: DispatchServerConfiguration = {
        let data = load_or_create_config("dispatch.json", DEFAULT_CONFIG);
        from_str(&data).unwrap()
    };
}
