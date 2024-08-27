use common::util::load_or_create_config;
use lazy_static::lazy_static;
use serde::Deserialize;
use serde_json::from_str;
use std::sync::{Arc, Mutex};

const DEFAULT_GLOBALS: &str = include_str!("../../gameplay.json");

// Define a mutable global state
lazy_static! {
    pub static ref INSTANCE: Arc<Mutex<Globals>> = {
        let data = load_or_create_config("gameplay.json", DEFAULT_GLOBALS);
        let globals: Globals = from_str(&data).unwrap();
        Arc::new(Mutex::new(globals))
    };
}

#[derive(Deserialize, Clone)]
pub struct Globals {
    pub multipath_config: MultipathConfig,
    pub monster_wave_list: Vec<Vec<u32>>,
}

#[derive(Deserialize, Clone)]
pub struct MultipathConfig {
    pub avatar_type: Vec<HeroPathTypeConfig>,
}

#[derive(Deserialize, Clone)]
pub struct HeroPathTypeConfig {
    pub avatar_id: i32,     // avatar id
    pub path: String, // path type
}

pub fn reload_gameplay_config() -> Result<(), Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string("gameplay.json")?;
    let new_instance: Globals = from_str(&data)?;

    let mut config = INSTANCE.lock().unwrap();
    *config = new_instance;

    Ok(())
}
