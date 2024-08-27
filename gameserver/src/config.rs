use common::{config::DatabaseConfig, util::load_or_create_config};
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::from_str;

use crate::game::gameplay_conf;

const DEFAULT_CONFIG: &str = include_str!("../gameserver.json");

#[derive(Deserialize)]
pub struct GameServerConfiguration {
    pub tcp_port: u16,
    pub database: DatabaseConfig,
}

lazy_static! {
    pub static ref CONFIGURATION: GameServerConfiguration = {
        let data = load_or_create_config("gameserver.json", DEFAULT_CONFIG);
        from_str(&data).unwrap()
    };
}

pub fn init_config() {
    // initialize statics
    let _configuration = &*CONFIGURATION;
    let _gameplay_conf = &*gameplay_conf;
}
