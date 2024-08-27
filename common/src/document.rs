use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UidIncrementDocument {
    pub inc_uid: u32,
    pub inc_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AccountDocument {
    pub username: String,
    pub account_uid: String,
    pub account_token: String,
    pub account_password: String,
}

#[derive(Serialize, Deserialize)]
pub struct ComboTokenDocument {
    pub uid: String,
    pub token: String,
    pub device_id: String,
}

#[derive(Serialize, Deserialize)]
pub struct PlayerDocument {
    pub uid: u32,
    pub account_uid: String,
    pub player_data_bin: String,
}
