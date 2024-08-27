use common::document::ComboTokenDocument;

use super::*;

pub async fn verify_combo_token(account_uid: &str, token: &str) -> Result<bool> {
    let collection = client()
        .database(&CONFIGURATION.database.name)
        .collection::<ComboTokenDocument>(&CONFIGURATION.database.combo_token_collection);

    let document = collection
        .find_one(
            doc! {
                "uid": account_uid,
                "token": token
            },
            None,
        )
        .await?;

    Ok(document.is_some())
}
