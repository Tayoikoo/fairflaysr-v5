use common::document::{PlayerDocument, UidIncrementDocument};
use mongodb::{
    bson::doc,
    options::{FindOneAndUpdateOptions, ReturnDocument},
};
use proto::PlayerDataBin;

use super::*;
use crate::util;

pub async fn update_player_bin(uid: u32, player_bin: &PlayerDataBin) -> Result<()> {
    let collection = client()
        .database(&CONFIGURATION.database.name)
        .collection::<PlayerDocument>(&CONFIGURATION.database.player_collection);

    let player_data_bin = util::serialize_player_bin(player_bin);
    collection
        .update_one(
            doc! {"uid": uid},
            doc! {"$set": {"player_data_bin": player_data_bin}},
            None,
        )
        .await?;

    Ok(())
}

pub async fn get_player_bin_by_account_uid(account_uid: &str) -> Result<(u32, PlayerDataBin)> {
    let collection = client()
        .database(&CONFIGURATION.database.name)
        .collection::<PlayerDocument>(&CONFIGURATION.database.player_collection);

    let document = collection
        .find_one(doc! {"account_uid": account_uid}, None)
        .await?;

    if let Some(document) = document {
        let player_bin = util::deserialize_player_bin(&document.player_data_bin)?;
        Ok((document.uid, player_bin))
    } else {
        let uid = next_uid().await?;
        let player_bin = PlayerDataBin::default();

        let document = PlayerDocument {
            uid,
            account_uid: account_uid.to_string(),
            player_data_bin: util::serialize_player_bin(&player_bin),
        };

        collection.insert_one(document, None).await?;
        Ok((uid, player_bin))
    }
}

pub async fn next_uid() -> Result<u32> {
    let collection = client()
        .database(&CONFIGURATION.database.name)
        .collection::<UidIncrementDocument>(&CONFIGURATION.database.autoincrement_collection);

    let inc_doc = collection
        .find_one_and_update(
            doc! {
                "inc_name": INCREMENT_NAME.to_string()
            },
            doc! {
                "$inc": {
                    "inc_uid": 10000
                }
            },
            FindOneAndUpdateOptions::builder()
                .return_document(ReturnDocument::After)
                .build(),
        )
        .await?;

    if let Some(document) = inc_doc {
        Ok(document.inc_uid)
    } else {
        collection
            .insert_one(
                UidIncrementDocument {
                    inc_uid: INITIAL_UID + 1,
                    inc_name: INCREMENT_NAME.to_string(),
                },
                None,
            )
            .await?;

        Ok(INITIAL_UID + 1)
    }
}
