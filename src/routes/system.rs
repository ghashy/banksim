use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing;
use axum::Json;
use axum::Router;
use fastwebsockets::upgrade;
use fastwebsockets::Frame;
use fastwebsockets::OpCode;
use fastwebsockets::WebSocketError;
use tokio::sync::TryLockError;

use crate::bank::BankOperationError;
use crate::bank::Transaction;
use crate::domain::requests::system_api::AddAccountRequest;
use crate::domain::requests::system_api::DeleteAccountRequest;
use crate::domain::requests::system_api::NewTransactionRequest;
use crate::domain::requests::system_api::OpenCreditRequest;
use crate::domain::responses::system_api::ListAccountsResponse;
use crate::domain::responses::system_api::{
    AddAccountResponse, ListCardTokensResponse,
};
use crate::error_chain_fmt;
use crate::middleware::BasicAuthLayer;
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
enum SystemApiError {
    #[error("Mutex lock error: {0}")]
    MutexLockError(#[from] TryLockError),
    #[error("Bank operation error: {0}")]
    BankOperationError(#[from] BankOperationError),
}

impl std::fmt::Debug for SystemApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

impl IntoResponse for SystemApiError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("System api error: {self}");
        match self {
            SystemApiError::MutexLockError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR.into_response()
            }
            SystemApiError::BankOperationError(_) => {
                StatusCode::BAD_REQUEST.into_response()
            }
        }
    }
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn system_router(state: AppState) -> Router {
    Router::new()
        .route("/account", routing::post(add_account))
        .route("/account", routing::delete(delete_account))
        .route("/list_accounts", routing::get(list_accounts))
        .route("/credit", routing::post(open_credit))
        .route("/transaction", routing::post(new_transaction))
        .route("/emission", routing::get(emission))
        .route("/store_card", routing::get(store_card))
        .route("/store_balance", routing::get(store_balance))
        .route("/list_transactions", routing::get(list_transactions))
        .route("/subscribe_on_accounts", routing::get(ws_accounts))
        .route("/subscribe_on_traces", routing::get(ws_traces))
        .with_state(state.clone())
        .layer(BasicAuthLayer { state })
}

#[tracing::instrument(name = "Add a new account to the bank", skip_all)]
async fn add_account(
    State(state): State<AppState>,
    Json(req): Json<AddAccountRequest>,
) -> Result<Json<AddAccountResponse>, SystemApiError> {
    let card_number = state.bank.handler().await.add_account(&req.password);
    Ok(Json(AddAccountResponse { card_number }))
}

#[tracing::instrument(name = "Delete existing account", skip_all)]
async fn delete_account(
    State(state): State<AppState>,
    Json(req): Json<DeleteAccountRequest>,
) -> Result<StatusCode, SystemApiError> {
    state.bank.handler().await.delete_account(req.card_number)?;
    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "List info about accounts", skip_all)]
async fn list_accounts(
    State(state): State<AppState>,
) -> Result<Json<ListAccountsResponse>, SystemApiError> {
    let accounts = state.bank.handler().await.list_accounts();
    Ok(Json(ListAccountsResponse { accounts }))
}

#[tracing::instrument(name = "Open credit for account", skip_all)]
async fn open_credit(
    State(state): State<AppState>,
    Json(req): Json<OpenCreditRequest>,
) -> Result<StatusCode, SystemApiError> {
    state
        .bank
        .handler()
        .await
        .open_credit(req.card_number, req.amount)?;
    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "Create a new transaction", skip_all)]
async fn new_transaction(
    State(state): State<AppState>,
    Json(req): Json<NewTransactionRequest>,
) -> Result<StatusCode, SystemApiError> {
    let mut bank_handler = state.bank.handler().await;
    let sender = bank_handler.find_account(&req.from)?;
    let receiver = bank_handler.find_account(&req.to)?;
    bank_handler.new_transaction(&sender, &receiver, req.amount)?;
    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "Get a vec with transactions", skip_all)]
async fn list_transactions(
    State(state): State<AppState>,
) -> Json<Vec<Transaction>> {
    Json(state.bank.handler().await.list_transactions())
}

#[tracing::instrument(name = "Get bank emission", skip_all)]
async fn emission(State(state): State<AppState>) -> String {
    state.bank.handler().await.bank_emission().to_string()
}

#[tracing::instrument(name = "Get store balance", skip_all)]
async fn store_balance(State(state): State<AppState>) -> String {
    state.bank.handler().await.store_balance().to_string()
}

#[tracing::instrument(name = "Get store card number", skip_all)]
async fn store_card(State(state): State<AppState>) -> String {
    state
        .bank
        .handler()
        .await
        .get_store_account()
        .card()
        .as_ref()
        .to_string()
}

#[tracing::instrument(name = "Register a ws accounts subscriber", skip_all)]
async fn ws_accounts(
    State(state): State<AppState>,
    ws: upgrade::IncomingUpgrade,
) -> impl IntoResponse {
    let (response, fut) = ws.upgrade().unwrap();

    tokio::task::spawn(async move {
        if let Err(e) = handle_accounts_subscriber(state, fut).await {
            eprintln!("Error in websocket connection: {}", e);
        }
    });

    response
}

#[tracing::instrument(name = "Register a ws traces subscriber", skip_all)]
async fn ws_traces(
    State(state): State<AppState>,
    ws: upgrade::IncomingUpgrade,
) -> impl IntoResponse {
    let (response, fut) = ws.upgrade().unwrap();
    state.ws_appender.add_subscriber(fut).await;
    response
}

// ───── Functions ────────────────────────────────────────────────────────── //

async fn handle_accounts_subscriber(
    state: AppState,
    fut: upgrade::UpgradeFut,
) -> Result<(), WebSocketError> {
    let mut ws = fastwebsockets::FragmentCollector::new(fut.await?);
    let mut rx = state.bank.handler().await.subscribe();

    loop {
        tokio::select! {
            notification = rx.changed() => {
                match notification {
                    Ok(()) => {
                        if let Err(e) = ws
                            .write_frame(Frame::new(
                                true,
                                OpCode::Text,
                                None,
                                fastwebsockets::Payload::Borrowed(&[]),
                            ))
                            .await
                        {
                            tracing::error!(
                                "Failed to notify client about bank locking: {e}"
                            );
                        }
                    }
                    Err(e) => {
                        tracing::error!(
                            "Failed to receive bank lock notification: {e}"
                        );
                        break;
                    }
                }

            }
            frame = ws.read_frame() => {
                // Assume connection is closed
                if frame.is_err() {
                    break;
                }
            }
        }
    }

    tracing::info!("End to serve ws account subscriber");
    Ok(())
}
