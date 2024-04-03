use axum::extract::State;
use axum::http::Uri;
use axum::routing;
use axum::Json;
use axum::Router;
use banksim_api::session::webhook::WebhookRequest;
use banksim_api::session::webhook::WebhookResponse;
use banksim_api::OperationError;
use banksim_api::OperationStatus;
use banksim_api::Tokenizable;

use crate::bank::Bank;
use crate::session::Session;
use crate::startup::AppState;

use self::init::init_router;

use super::html_pages_and_triggers::Credentials;

pub mod init;

trait WebhookHandler {
    async fn handle(
        bank: Bank,
        session: &Session,
        req: &WebhookRequest,
    ) -> Result<(), Json<WebhookResponse>>;
}

struct ConfirmWebhook;
struct CaptureWebhook;
struct CancelWebhook;

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn session_router() -> Router<AppState> {
    Router::new()
        .route("/confirm", routing::post(webhook::<ConfirmWebhook>))
        .route("/capture", routing::post(webhook::<CaptureWebhook>))
        .route("/cancel", routing::post(webhook::<CancelWebhook>))
        .nest("/init", init_router())
}

#[tracing::instrument(name = "Webhook request", skip_all, fields(uri=?uri))]
async fn webhook<T>(
    State(state): State<AppState>,
    uri: Uri,
    Json(req): Json<WebhookRequest>,
) -> Result<Json<WebhookResponse>, Json<WebhookResponse>>
where
    T: WebhookHandler,
{
    let session = acquire_session(
        &state,
        req.session_id,
        WebhookResponse {
            session_id: req.session_id,
            status: banksim_api::OperationStatus::Fail(
                OperationError::SessionNotFound,
            ),
        },
    )?;

    T::handle(state.bank, &session, &req).await?;

    // Successfully updated session state
    Ok(Json(WebhookResponse {
        session_id: req.session_id,
        status: OperationStatus::Success,
    }))
}

impl WebhookHandler for ConfirmWebhook {
    async fn handle(
        bank: Bank,
        session: &Session,
        req: &WebhookRequest,
    ) -> Result<(), Json<WebhookResponse>> {
        match session {
            Session::PaymentSession(p_s) => {
                use crate::session::payment::Event;
                use crate::session::payment::State as PaymentState;

                let mut guard = p_s.state.lock().await;
                validate_session(&guard.store_credentials, req).await?;
                if let Err(e) = match guard.state() {
                    PaymentState::ReadyToConfirm {} => {
                        guard.handle(&Event::ConfirmRequest).await;
                        Ok(())
                    }
                    PaymentState::Closed { .. } => {
                        Err(OperationStatus::Fail(OperationError::Cancelled))
                    }
                    PaymentState::Failed { err, .. } => {
                        Err(OperationStatus::Fail(err.clone()))
                    }
                    _ => Err(OperationStatus::Fail(OperationError::BadRequest)),
                } {
                    return Err(Json(WebhookResponse {
                        session_id: req.session_id,
                        status: e,
                    }));
                }
            }
            Session::CardTokenRegSession(t_s) => {
                use crate::session::card_token::Event;
                use crate::session::card_token::State as TokenState;

                let mut guard = t_s.state.lock().await;
                validate_session(&guard.store_credentials, req).await?;
                if let Err(e) = match guard.state() {
                    TokenState::ReadyToConfirm {} => {
                        guard.handle(&Event::ConfirmRequest { bank }).await;
                        Ok(())
                    }
                    TokenState::Closed { .. } => {
                        Err(OperationStatus::Fail(OperationError::Cancelled))
                    }
                    TokenState::Failed { err, .. } => {
                        Err(OperationStatus::Fail(err.clone()))
                    }
                    _ => Err(OperationStatus::Fail(OperationError::BadRequest)),
                } {
                    return Err(Json(WebhookResponse {
                        session_id: req.session_id,
                        status: e,
                    }));
                }
            }
        }
        Ok(())
    }
}

impl WebhookHandler for CaptureWebhook {
    async fn handle(
        bank: Bank,
        session: &Session,
        req: &WebhookRequest,
    ) -> Result<(), Json<WebhookResponse>> {
        match session {
            Session::PaymentSession(p_s) => {
                use crate::session::payment::Event;
                use crate::session::payment::State as PaymentState;

                let mut guard = p_s.state.lock().await;
                validate_session(&guard.store_credentials, req).await?;
                if let Err(e) = match guard.state() {
                    PaymentState::ReadyToCapture {} => {
                        guard.handle(&Event::CaptureRequest { bank }).await;
                        Ok(())
                    }
                    PaymentState::Closed { .. } => {
                        Err(OperationStatus::Fail(OperationError::Cancelled))
                    }
                    PaymentState::Failed { err, .. } => {
                        Err(OperationStatus::Fail(err.clone()))
                    }
                    _ => Err(OperationStatus::Fail(OperationError::BadRequest)),
                } {
                    return Err(Json(WebhookResponse {
                        session_id: req.session_id,
                        status: e,
                    }));
                }
            }
            Session::CardTokenRegSession(_) => {
                return Err(Json(WebhookResponse {
                    session_id: req.session_id,
                    status: OperationStatus::Fail(OperationError::BadRequest),
                }));
            }
        }
        Ok(())
    }
}

impl WebhookHandler for CancelWebhook {
    async fn handle(
        _: Bank,
        session: &Session,
        req: &WebhookRequest,
    ) -> Result<(), Json<WebhookResponse>> {
        match session {
            Session::PaymentSession(p_s) => {
                use crate::session::payment::Event;
                use crate::session::payment::State as PaymentState;

                let mut guard = p_s.state.lock().await;
                validate_session(&guard.store_credentials, req).await?;
                if let Err(e) = match guard.state() {
                    PaymentState::Closed { .. } => {
                        Err(OperationStatus::Fail(OperationError::Cancelled))
                    }
                    PaymentState::Failed { err, .. } => {
                        Err(OperationStatus::Fail(err.clone()))
                    }
                    PaymentState::Successed { .. } => {
                        Err(OperationStatus::Fail(OperationError::BadRequest))
                    }
                    _ => {
                        guard.handle(&Event::CancelRequest).await;
                        Ok(())
                    }
                } {
                    return Err(Json(WebhookResponse {
                        session_id: req.session_id,
                        status: e,
                    }));
                }
            }
            Session::CardTokenRegSession(t_s) => {
                use crate::session::card_token::Event;
                use crate::session::card_token::State as TokenState;

                let mut guard = t_s.state.lock().await;
                validate_session(&guard.store_credentials, req).await?;
                if let Err(e) = match guard.state() {
                    TokenState::Closed { .. } => {
                        Err(OperationStatus::Fail(OperationError::Cancelled))
                    }
                    TokenState::Failed { err, .. } => {
                        Err(OperationStatus::Fail(err.clone()))
                    }
                    TokenState::Successed { .. } => {
                        Err(OperationStatus::Fail(OperationError::BadRequest))
                    }
                    _ => {
                        guard.handle(&Event::CancelRequest).await;
                        Ok(())
                    }
                } {
                    return Err(Json(WebhookResponse {
                        session_id: req.session_id,
                        status: e,
                    }));
                }
            }
        }
        Ok(())
    }
}

// ───── Helpers ──────────────────────────────────────────────────────────── //

/// Try acquire session by id
fn acquire_session<T>(
    state: &AppState,
    id: uuid::Uuid,
    err_response: T,
) -> Result<Session, Json<T>> {
    match state.sessions.try_acquire_session_by_id(id) {
        Ok(session) => Ok(session),
        Err(e) => {
            // No such payment
            tracing::error!("Session not found: {e}");
            Err(Json(err_response))
        }
    }
}

/// Validate store credentials
async fn validate_session(
    store_creds: &Credentials,
    req: &WebhookRequest,
) -> Result<(), Json<WebhookResponse>> {
    if req.validate_token(&store_creds.password).is_err() {
        Err(Json(WebhookResponse {
            session_id: req.session_id,
            status: banksim_api::OperationStatus::Fail(
                OperationError::NotAuthorizedRequest,
            ),
        }))
    } else {
        Ok(())
    }
}
