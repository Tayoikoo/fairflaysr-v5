use common::document::{AccountDocument, UidIncrementDocument};
use mongodb::options::{FindOneAndUpdateOptions, ReturnDocument};
use rand::distributions::{Alphanumeric, DistString};

use super::*;

pub enum AccountCreationStatus {
    Success,
    AlreadyExists,
}

pub async fn get_account_by_uid(
    client: &Client,
    account_uid: &str,
) -> Result<Option<AccountDocument>> {
    let collection = client
        .database(&CONFIGURATION.database.name)
        .collection(&CONFIGURATION.database.sdk_account_collection);

    let document = collection
        .find_one(doc! { "account_uid": account_uid }, None)
        .await?;

    Ok(document)
}

pub async fn get_account_by_name(
    client: &Client,
    username: &str,
) -> Result<Option<AccountDocument>> {
    let collection = client
        .database(&CONFIGURATION.database.name)
        .collection(&CONFIGURATION.database.sdk_account_collection);

    let document = collection
        .find_one(doc! { "username": username }, None)
        .await?;

    Ok(document)
}

pub async fn create_account(
    client: &Client,
    username: &str,
    password: &str,
) -> Result<AccountCreationStatus> {
    let collection = client
        .database(&CONFIGURATION.database.name)
        .collection::<AccountDocument>(&CONFIGURATION.database.sdk_account_collection);

    if collection
        .count_documents(doc! { "username": username }, None)
        .await?
        != 0
    {
        return Ok(AccountCreationStatus::AlreadyExists);
    }

    let uid = next_uid(client).await?;
    let document = AccountDocument {
        account_uid: uid.to_string(),
        username: username.to_string(),
        account_password: password.to_string(),
        account_token: Alphanumeric.sample_string(&mut rand::thread_rng(), 40),
    };

    collection.insert_one(&document, None).await?;

    tracing::info!("New account registered, username: {username}, uid: {uid}");
    Ok(AccountCreationStatus::Success)
}

pub async fn next_uid(client: &Client) -> Result<u32> {
    let collection = client
        .database(&CONFIGURATION.database.name)
        .collection::<UidIncrementDocument>(&CONFIGURATION.database.autoincrement_collection);

    let inc_doc = collection
        .find_one_and_update(
            doc! {
                "inc_name": INCREMENT_NAME.to_string()
            },
            doc! {
                "$inc": {
                    "inc_uid": 1
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
