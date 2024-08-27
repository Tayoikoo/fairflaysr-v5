use serde::Deserialize;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub connection_string: String,
    pub name: String,
    pub autoincrement_collection: String,
    pub sdk_account_collection: String,
    pub combo_token_collection: String,
    pub player_collection: String,
}
