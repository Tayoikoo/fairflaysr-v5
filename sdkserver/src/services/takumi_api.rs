use super::*;
use axum::Json;
use serde_json::json;

const RISKY_CHECK: &str = "/account/risky/api/check";

pub fn setup_routes(router: Router<SdkContext>) -> Router<SdkContext> {
    router.route(RISKY_CHECK, post(risky_check))
}

#[tracing::instrument]
async fn risky_check() -> Json<serde_json::Value> {
    Json(json!({
        "data": {},
        "message": "OK",
        "retcode": 0
    }))
}
