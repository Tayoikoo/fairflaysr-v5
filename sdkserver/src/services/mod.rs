use crate::SdkContext;
use axum::routing::{get, post};
use axum::{extract::State, Json, Router};
use serde_json::json;

pub mod granter;
pub mod mdk_shield;
pub mod pages;
pub mod reverse_proxy;
pub mod takumi_api;

fn fail_json(retcode: i32, message: &str) -> Json<serde_json::Value> {
    Json(json! ({
        "retcode": retcode,
        "message": message
    }))
}
