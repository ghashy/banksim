use std::time::Duration;

use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use axum::{routing, Json, Router};
use banksim_api::token_info::{self, TokenInfoRequest, TokenInfoResponse};
use banksim_api::Tokenizable;
use futures::FutureExt;
use secrecy::Secret;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::card_number::CardNumber;
use crate::html_gen::{SubmitCardNumberPage, SubmitPaymentPage};
use crate::startup::AppState;

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn token_router() -> Router<AppState> {
    Router::new().route("/info", routing::get(get_token_info))
}

#[tracing::instrument(skip_all)]
async fn get_token_info(
    State(state): State<AppState>,
    Json(req): Json<TokenInfoRequest>,
) -> Json<TokenInfoResponse> {
    let password = &state.settings.terminal_settings.password;
    if req.validate_token(password).is_err() {
        return Json(TokenInfoResponse {
            status: Err("Not authorized request".to_string()),
        });
    }
    Json(
        match state.bank.get_account_by_token(&req.card_token).await {
            Ok(acc) => {
                if acc.is_existing {
                    TokenInfoResponse { status: Ok(true) }
                } else {
                    TokenInfoResponse { status: Ok(false) }
                }
            }
            Err(e) => TokenInfoResponse {
                status: Err("No token found".to_string()),
            },
        },
    )
}
