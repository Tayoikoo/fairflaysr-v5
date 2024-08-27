use super::*;
use axum::{
    response::{Html, Redirect},
    Form,
};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Deserialize;

use crate::{
    database::{self, AccountCreationStatus},
    util, SdkContext,
};

pub const ACCOUNT_REGISTER: &str = "/account/register";

const REGISTRATION_PAGE_HTML: &str = include_str!("../../../html/registration_page.html");
const REGISTRATION_RESULT_HTML: &str = include_str!("../../../html/registration_result.html");

pub async fn registration_page() -> Html<String> {
    Html(REGISTRATION_PAGE_HTML.to_string())
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
    pub password_v2: String,
}

pub fn setup_routes(router: Router<SdkContext>) -> Router<SdkContext> {
    router
        .route(ACCOUNT_REGISTER, get(registration_page))
        .route(ACCOUNT_REGISTER, post(process_register))
        .route("/", get(|| async { Redirect::permanent(ACCOUNT_REGISTER) }))
}

lazy_static! {
    static ref ALLOWED_USERNAME: Regex = Regex::new("^[a-zA-Z0-9._@-]{6,25}$").unwrap();
}

pub async fn process_register(
    State(context): State<SdkContext>,
    Form(register): Form<RegisterRequest>,
) -> Html<String> {
    if !ALLOWED_USERNAME.is_match(&register.username) {
        return html_result("error", "Invalid username format; should consists of characters [A-Za-z0-9_] and be at least 6 characters long.");
    }

    if !(8..30).contains(&register.password.len()) {
        return html_result(
            "error",
            "Password should contain at least 8 and not more than 30 characters",
        );
    }

    if register.password != register.password_v2 {
        return html_result("error", "Passwords do not match");
    }

    let Ok(password_hash) = util::hash_password(&register.password) else {
        return html_result("error", "Invalid password input");
    };

    let Ok(status) =
        database::create_account(&context.db_client, &register.username, &password_hash).await
    else {
        return html_result("error", "Internal server error");
    };

    match status {
        AccountCreationStatus::Success => html_result(
            "success",
            "Successfully registered. Now you can use in-game login.",
        ),
        AccountCreationStatus::AlreadyExists => {
            html_result("error", "Account with specified username already exists.")
        }
    }
}

fn html_result(result: &str, message: &str) -> Html<String> {
    Html(
        REGISTRATION_RESULT_HTML
            .replace("%RESULT%", result)
            .replace("%MESSAGE%", message),
    )
}
