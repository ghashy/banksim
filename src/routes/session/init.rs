use axum::{extract::State, routing, Json, Router};
use banksim_api::make_payment::{MakePaymentRequest, MakePaymentResponse};
use banksim_api::OperationError;
use banksim_api::{Operation, Tokenizable};

use serde::Serialize;
use url::Url;

use banksim_api::init_payment::{InitPaymentRequest, InitPaymentResponse};
use banksim_api::register_card_token::{
    RegisterCardTokenRequest, RegisterCardTokenResponse,
};

use crate::routes::html_pages_and_triggers::Credentials;
use crate::session::IntoSession;
use crate::startup::AppState;
use crate::tasks::wait_hour_and_remove;

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn init_router() -> Router<AppState> {
    Router::new()
        .route(
            "/payment",
            routing::post(
                init_session::<InitPaymentRequest, InitPaymentResponse>,
            ),
        )
        .route(
            "/card_token_reg",
            routing::post(
                init_session::<
                    RegisterCardTokenRequest,
                    RegisterCardTokenResponse,
                >,
            ),
        )
        .route("/MakePayment", routing::post(make_payment))
}

#[tracing::instrument(name = "Init session", skip_all)]
async fn init_session<Request, Response>(
    State(mut state): State<AppState>,
    Json(req): Json<Request>,
) -> Json<impl Serialize + 'static>
where
    Request: Tokenizable + IntoSession,
    Response: Operation + Serialize + 'static,
{
    // NOTE: we have only one store account in our virtual bank
    let store_card = match state.bank.get_store_account().await {
        Ok(acc) => acc.card(),
        Err(e) => {
            tracing::error!("Failed to get store account: {e}");
            return Json(Response::operation_error(
                OperationError::Unexpected(e.to_string()),
            ));
        }
    };

    let store_creds = Credentials {
        card_number: store_card,
        password: state.settings.terminal_settings.password.clone(),
    };

    // Authorize request
    if req
        .validate_token(&state.settings.terminal_settings.password)
        .is_err()
    {
        tracing::warn!("Unauthorized request");
        return Json(Response::operation_error(
            OperationError::NotAuthorizedRequest,
        ));
    }

    let (tx, rx) = tokio::sync::oneshot::channel();

    let session =
        req.create_session(store_creds, state.http_client.clone(), tx);
    let session_id = session.id();
    let created_at = session.creation_time();

    // We store active sessions in the RAM for simplicity
    match state.sessions.insert(session) {
        Ok(result) => result,
        Err(e) => {
            tracing::error!("Failed to initiate session: {e}");
            return Json(Response::operation_error(
                OperationError::Unexpected("Internal error".to_string()),
            ));
        }
    };

    // Launch async task which will track our session
    wait_hour_and_remove(state.sessions, rx, session_id, created_at);

    let url = format!(
        "http://{}:{}/{}/{}",
        state.settings.addr,
        state.settings.port,
        Request::page_endpoint(),
        session_id
    );

    let session_ui_url = match Url::parse(&url) {
        Ok(url) => url,
        Err(e) => {
            tracing::error!("Failed to parse url: {e}");
            return Json(Response::operation_error(
                OperationError::Unexpected("Internal error".to_string()),
            ));
        }
    };

    Json(Response::operation_success(session_ui_url, session_id))
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

    // We have only one store account in our virtual bank
    let store_card = match state.bank.get_store_account().await {
        Ok(acc) => acc.card(),
        Err(e) => {
            tracing::error!("Failed to get store account: {e}");
            return Json(MakePaymentResponse::err(e.to_string()));
        }
    };

    let recipient_card =
        match state.bank.get_account_by_token(&req.recipient_token).await {
            Ok(acc) => acc.card(),
            Err(e) => {
                tracing::error!("Failed to find account by card token: {e}");
                return Json(MakePaymentResponse::err(e.to_string()));
            }
        };

    match state
        .bank
        .new_transaction(&store_card, &recipient_card, req.amount)
        .await
    {
        Ok(()) => Json(MakePaymentResponse::success()),
        Err(e) => {
            tracing::error!("Failed to make transaction: {e}");
            Json(MakePaymentResponse::err(e.to_string()))
        }
    }
}
