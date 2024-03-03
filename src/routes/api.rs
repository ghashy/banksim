use axum::{extract::State, routing, Json, Router};
use banksim_api::make_payment::{MakePaymentRequest, MakePaymentResponse};
use banksim_api::{Operation, Tokenizable};

use serde::Serialize;
use url::Url;

use banksim_api::init_payment::{InitPaymentRequest, InitPaymentResponse};
use banksim_api::register_card_token::{
    RegisterCardTokenRequest, RegisterCardTokenResponse,
};

use crate::interaction_sessions::IntoSession;
use crate::{startup::AppState, tasks::wait_and_remove};

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn api_router(state: AppState) -> Router {
    Router::new()
        .route(
            "/InitPayment",
            routing::post(
                init_session::<InitPaymentRequest, InitPaymentResponse>,
            ),
        )
        .route(
            "/InitCardTokenRegistration",
            routing::post(
                init_session::<
                    RegisterCardTokenRequest,
                    RegisterCardTokenResponse,
                >,
            ),
        )
        .route("/MakePayment", routing::post(make_payment))
        .with_state(state)
}

#[tracing::instrument(name = "Init session", skip_all)]
async fn init_session<Request, Response>(
    State(state): State<AppState>,
    Json(req): Json<Request>,
) -> Json<impl Serialize + 'static>
where
    Request: Tokenizable + IntoSession,
    Response: Operation + Serialize + 'static,
{
    // Authorize request
    if req
        .validate_token(&state.settings.terminal_settings.password)
        .is_err()
    {
        tracing::warn!("Unauthorized request");
        return Json(Response::operation_error("Unauthorized".to_string()));
    }

    let bank_handler = state.bank.handler().await;

    // We have only one store account in our virtual bank
    let store_card = bank_handler.get_store_account().card();

    let session = req.create_session(store_card);
    let id = session.id();
    let created_at = session.creation_time;

    // We store active sessions in the RAM for simplicity
    match state.interaction_sessions.insert(session) {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to initiate session: {e}");
            return Json(Response::operation_error(
                "Internal error".to_string(),
            ));
        }
    };

    // Launch async task which will track our session
    wait_and_remove(state.interaction_sessions, id, created_at);

    let url = format!(
        "{}:{}/{}/{}",
        state.settings.addr,
        state.settings.port,
        Request::page_endpoint(),
        id
    );

    let session_ui_url = match Url::parse(&url) {
        Ok(url) => url,
        Err(e) => {
            tracing::error!("Failed to parse url: {e}");
            return Json(Response::operation_error(
                "Internal error".to_string(),
            ));
        }
    };

    Json(Response::operation_success(session_ui_url))
}

#[tracing::instrument(name = "Make payment", skip_all)]
async fn make_payment(
    State(state): State<AppState>,
    Json(req): Json<MakePaymentRequest>,
) -> Json<MakePaymentResponse> {
    // Authorize request
    if req
        .validate_token(&state.settings.terminal_settings.password)
        .is_err()
    {
        tracing::warn!("Unauthorized request");
        return Json(MakePaymentResponse::err("Unauthorized".to_string()));
    }

    let mut bank_handler = state.bank.handler().await;

    // We have only one store account in our virtual bank
    let store = bank_handler.get_store_account();

    let recipient =
        match bank_handler.get_account_by_token(&req.recipient_token) {
            Ok(acc) => acc,
            Err(e) => {
                tracing::error!("Failed to find account by card token: {e}");
                return Json(MakePaymentResponse::err(e.to_string()));
            }
        };

    match bank_handler.new_transaction(&store, &recipient, req.amount) {
        Ok(()) => Json(MakePaymentResponse::success()),
        Err(e) => {
            tracing::error!("Failed to make transaction: {e}");
            Json(MakePaymentResponse::err(e.to_string()))
        }
    }
}
