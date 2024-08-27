use anyhow::Result;
use mongodb::{bson::doc, Client};

use crate::config::CONFIGURATION;

mod account_collection;
mod combo_token;

pub use account_collection::*;
pub use combo_token::*;

const INCREMENT_NAME: &str = "sdk_account_uid";
const INITIAL_UID: u32 = 100000;
