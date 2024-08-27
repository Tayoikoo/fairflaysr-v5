use serde::Deserialize;
use serde_json::{from_str, json};

use crate::{database, SdkContext};

use super::*;

const COMBO_LOGIN_V2: &str = "/:product_name/combo/granter/login/v2/login";

#[derive(Deserialize)]
struct RequestData {
    pub uid: String,
    pub token: String,
}

#[derive(Deserialize)]
struct GranterTokenRequest {
    pub data: String,
    pub device: String,
}

pub fn setup_routes(router: Router<SdkContext>) -> Router<SdkContext> {
    router.route(COMBO_LOGIN_V2, post(combo_login_v2))
}

async fn combo_login_v2(
    State(context): State<SdkContext>,
    Json(request): Json<GranterTokenRequest>,
) -> Json<serde_json::Value> {
    let Ok(data) = from_str::<RequestData>(&request.data) else {
        return fail_json(-101, "Account token error");
    };

    match database::get_combo_token(&context.db_client, &data.uid, &data.token, &request.device)
        .await
    {
        Ok(Some(token)) => Json(json!({
            "data": {
                "account_type": 1,
                "combo_id": data.uid,
                "combo_token": token,
                "data": "{\"guest\":false}",
                "heartbeat": false,
                "open_id": data.uid
            },
            "message": "OK",
            "retcode": 0
        })),
        Ok(None) => fail_json(-101, "Account token error"),
        Err(_) => fail_json(-1, "Internal server error"),
    }
}
