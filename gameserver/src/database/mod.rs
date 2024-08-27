use anyhow::Result;
use mongodb::{bson::doc, Client};
use tokio::sync::OnceCell;

use crate::config::CONFIGURATION;

mod combo_token;
mod player_collection;

pub use combo_token::*;
pub use player_collection::*;

const INCREMENT_NAME: &str = "player_uid";
const INITIAL_UID: u32 = 0;

static CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn init() -> Result<()> {
    let client = Client::with_uri_str(&CONFIGURATION.database.connection_string).await?;
    CLIENT.set(client)?;

    Ok(())
}

// mongodb::Client uses Arc internally, so it's safe to clone it
// and share between multiple tasks/threads
fn client() -> Client {
    CLIENT.get().unwrap().clone()
}
