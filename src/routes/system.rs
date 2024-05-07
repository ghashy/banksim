use axum::body::Body;
use axum::extract::Path;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::response::Response;
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
use crate::domain::responses::system_api::AddAccountResponse;
use crate::domain::responses::system_api::ListAccountsResponse;
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
    #[error("Not authorized request")]
    NotAuthorized,
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
            SystemApiError::BankOperationError(e) => Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from(e.to_string()))
                .unwrap_or(StatusCode::BAD_REQUEST.into_response()),
            SystemApiError::NotAuthorized => {
                StatusCode::UNAUTHORIZED.into_response()
            }
        }
    }
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn system_router(state: AppState) -> Router<AppState> {
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
        .route("/ws_token", routing::get(get_ws_token))
        .layer(BasicAuthLayer { state })
        .route("/subscribe_on_accounts/:token", routing::get(ws_accounts))
        .route("/subscribe_on_traces/:token", routing::get(ws_traces))
}

#[tracing::instrument(name = "Retrieve a new ws token", skip_all)]
async fn get_ws_token(State(app_state): State<AppState>) -> String {
    let token = uuid::Uuid::new_v4();
    app_state.ws_tokens.lock().await.insert(token.clone());
    token.to_string()
}

#[tracing::instrument(name = "Add a new account to the bank", skip_all)]
async fn add_account(
    State(state): State<AppState>,
    Json(req): Json<AddAccountRequest>,
) -> Result<Json<AddAccountResponse>, SystemApiError> {
    let card_number =
        state.bank.add_account(&req.username, &req.password).await?;
    Ok(Json(AddAccountResponse { card_number }))
}

#[tracing::instrument(name = "Delete existing account", skip_all)]
async fn delete_account(
    State(state): State<AppState>,
    Json(req): Json<DeleteAccountRequest>,
) -> Result<StatusCode, SystemApiError> {
    state.bank.delete_account(&req.card_number).await?;
    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "List info about accounts", skip_all)]
async fn list_accounts(
    State(state): State<AppState>,
) -> Result<Json<ListAccountsResponse>, SystemApiError> {
    let accounts = state.bank.list_accounts().await?;
    Ok(Json(ListAccountsResponse { accounts }))
}

#[tracing::instrument(name = "Open credit for account", skip_all)]
async fn open_credit(
    State(state): State<AppState>,
    Json(req): Json<OpenCreditRequest>,
) -> Result<StatusCode, SystemApiError> {
    state.bank.open_credit(&req.card_number, req.amount).await?;
    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "Create a new transaction", skip_all)]
async fn new_transaction(
    State(state): State<AppState>,
    Json(req): Json<NewTransactionRequest>,
) -> Result<StatusCode, SystemApiError> {
    state
        .bank
        .new_transaction(&req.from, &req.to, req.amount)
        .await?;
    Ok(StatusCode::OK)
}

#[tracing::instrument(name = "Get a vec with transactions", skip_all)]
async fn list_transactions(
    State(state): State<AppState>,
) -> Result<Json<Vec<Transaction>>, SystemApiError> {
    Ok(Json(state.bank.list_transactions().await?))
}

#[tracing::instrument(name = "Get bank emission", skip_all)]
async fn emission(
    State(state): State<AppState>,
) -> Result<String, SystemApiError> {
    Ok(state.bank.bank_emission().await?.to_string())
}

#[tracing::instrument(name = "Get store balance", skip_all)]
async fn store_balance(
    State(state): State<AppState>,
) -> Result<String, SystemApiError> {
    Ok(state.bank.store_balance().await?.to_string())
}

#[tracing::instrument(name = "Get store card number", skip_all)]
async fn store_card(
    State(state): State<AppState>,
) -> Result<String, SystemApiError> {
    Ok(state
        .bank
        .get_store_account()
        .await?
        .card()
        .as_ref()
        .to_string())
}

#[tracing::instrument(name = "Register a ws accounts subscriber", skip_all)]
async fn ws_accounts(
    State(state): State<AppState>,
    Path(ws_token): Path<uuid::Uuid>,
    ws: upgrade::IncomingUpgrade,
) -> Result<impl IntoResponse, SystemApiError> {
    if !state.ws_tokens.lock().await.remove(&ws_token) {
        return Err(SystemApiError::NotAuthorized);
    }
    let (response, fut) = ws.upgrade().unwrap();

    tokio::task::spawn(async move {
        if let Err(e) = handle_accounts_subscriber(state, fut).await {
            eprintln!("Error in websocket connection: {}", e);
        }
    });

    Ok(response)
}

#[tracing::instrument(name = "Register a ws traces subscriber", skip_all)]
async fn ws_traces(
    State(state): State<AppState>,
    Path(ws_token): Path<uuid::Uuid>,
    ws: upgrade::IncomingUpgrade,
) -> impl IntoResponse {
    if !state.ws_tokens.lock().await.remove(&ws_token) {
        return Err(SystemApiError::NotAuthorized);
    }

    let (response, fut) = ws.upgrade().unwrap();
    state.ws_appender.add_subscriber(fut).await;

    Ok(response)
}

// ───── Functions ────────────────────────────────────────────────────────── //

async fn handle_accounts_subscriber(
    state: AppState,
    fut: upgrade::UpgradeFut,
) -> Result<(), WebSocketError> {
    let mut ws = fastwebsockets::FragmentCollector::new(fut.await?);
    let mut rx = state.bank.subscribe().await;

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
