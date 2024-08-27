use common::document::ComboTokenDocument;
use rand::distributions::{Alphanumeric, DistString};

use super::*;

pub async fn get_combo_token(
    client: &Client,
    uid: &str,
    token: &str,
    device_id: &str,
) -> Result<Option<String>> {
    match get_account_by_uid(client, uid).await? {
        Some(account) if account.account_token == token => {}
        _ => return Ok(None),
    }

    let collection = client
        .database(&CONFIGURATION.database.name)
        .collection::<ComboTokenDocument>(&CONFIGURATION.database.combo_token_collection);

    if let Some(document) = collection
        .find_one(doc! { "uid": uid, "device_id": device_id }, None)
        .await?
    {
        Ok(Some(document.token))
    } else {
        let token = Alphanumeric.sample_string(&mut rand::thread_rng(), 40);
        collection
            .insert_one(
                ComboTokenDocument {
                    uid: uid.to_string(),
                    token: token.clone(),
                    device_id: device_id.to_string(),
                },
                None,
            )
            .await?;

        Ok(Some(token))
    }
}
