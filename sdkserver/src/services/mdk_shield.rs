use super::*;
use common::document::AccountDocument;
use serde::Deserialize;
use serde_json::json;

use crate::{config::CONFIGURATION, database, util, SdkContext};

const LOGIN: &str = "/:product_name/mdk/shield/api/login";
const VERIFY: &str = "/:product_name/mdk/shield/api/verify";

#[derive(Deserialize)]
struct LoginRequest {
    pub account: String,
    pub password: String,
    pub is_crypto: bool,
}

#[derive(Deserialize)]
struct VerifyRequest {
    pub uid: String,
    pub token: String,
}

pub fn setup_routes(router: Router<SdkContext>) -> Router<SdkContext> {
    router
        .route(LOGIN, post(login))
        .route(VERIFY, post(verify_token))
}

async fn login(
    State(context): State<SdkContext>,
    Json(request): Json<LoginRequest>,
) -> Json<serde_json::Value> {
    if !request.is_crypto {
        return fail_json(
            -10,
            "Invalid account format\r\nUnencrypted passwords are disabled by SDK security policy",
        );
    }

    let mut password_opt: Option<String> = None;
    if !CONFIGURATION.disable_password_check
    {
        if let Ok(password) = util::decrypt_string(&request.password) {
            password_opt = Some(password);
        } else {
            return fail_json(-10, "Your patch is outdated.\r\nGet new one at https://discord.gg/reversedrooms\r\n(Password decryption failed)");
        };
    }

    let account = match database::get_account_by_name(&context.db_client, &request.account).await {
        Ok(Some(account)) => account,
        Ok(None) => return fail_json(-101, "Account or password error (Check 1)"),
        Err(_) => return fail_json(-1, "Internal server error"),
    };

    if !CONFIGURATION.disable_password_check && util::verify_password(&password_opt.unwrap(), &account.account_password).is_err() {
        return fail_json(-101, "Account or password error (Check 2)");
    }

    success_json(account)
}

async fn verify_token(
    State(context): State<SdkContext>,
    Json(request): Json<VerifyRequest>,
) -> Json<serde_json::Value> {
    let account = match database::get_account_by_uid(&context.db_client, &request.uid).await {
        Ok(Some(account)) => account,
        Ok(None) => return fail_json(-101, "Account cache error"),
        Err(_) => return fail_json(-1, "Internal server error"),
    };

    if account.account_token != request.token {
        return fail_json(-101, "Account cache error");
    }

    success_json(account)
}

fn success_json(account: AccountDocument) -> Json<serde_json::Value> {
    Json(json!({
        "data": {
            "account": {
                "area_code": "**",
                "email": account.username,
                "country": "RU",
                "is_email_verify": "1",
                "token": account.account_token,
                "uid": account.account_uid
            },
            "device_grant_required": false,
            "reactivate_required": false,
            "realperson_required": false,
            "safe_mobile_required": false
        },
        "message": "OK",
        "retcode": 0
    }))
}
