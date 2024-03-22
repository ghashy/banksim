use std::time::Duration;

use askama::Template;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::Html;
use axum::{routing, Json, Router};
use secrecy::Secret;
use serde::Deserialize;
use uuid::Uuid;

use crate::domain::card_number::CardNumber;
use crate::html_gen::{SubmitCardNumberPage, SubmitPaymentPage};
use crate::startup::AppState;

// ───── Types ────────────────────────────────────────────────────────────── //

#[derive(Clone, Debug, Deserialize)]
pub struct Credentials {
    pub card_number: CardNumber,
    pub password: Secret<String>,
}

// ───── Handlers ─────────────────────────────────────────────────────────── //

pub fn pages_and_triggers_router() -> Router<AppState> {
    Router::new()
        // Payment page
        .route("/payment_page/:id", routing::get(payment_html_page))
        // Payment trigger
        .route("/payment/:id", routing::post(trigger_payment))
        // Card token page
        .route(
            "/register_card_token_page/:id",
            routing::get(card_token_registration_html_page),
        )
        // Card token trigger
        .route(
            "/card_token/:id",
            routing::post(trigger_card_token_registration),
        )
}

#[tracing::instrument(name = "Get payment html page", skip_all)]
pub async fn payment_html_page(
    State(state): State<AppState>,
    Path(payment_id): Path<Uuid>,
) -> Result<Html<String>, StatusCode> {
    // Try to create submit payment url for client (browser)
    let submit_payment_url = match format!(
        "http://{}:{}/payment/{}",
        state.settings.addr, state.settings.port, payment_id
    )
    .parse()
    {
        Ok(url) => url,
        Err(e) => {
            tracing::error!("Failed to parse string as url: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Try to return html payment page
    // We take lock on payment session lock here
    match state.sessions.try_acquire_session_by_id(payment_id) {
        Ok(session) => {
            let req = &session
                .payment_session()
                .ok_or(StatusCode::NOT_FOUND)?
                .state
                .lock()
                .await
                .req;
            match SubmitPaymentPage::new(req.amount, submit_payment_url)
                .render()
            {
                Ok(body) => Ok(Html(body)),
                Err(e) => {
                    tracing::error!("Failed to render payment html page: {e}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to get payment html page: {e}");
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// Handle payment, actually.
///
/// We return `String` with redirection url.
#[tracing::instrument(name = "Trigger payment", skip_all)]
pub async fn trigger_payment(
    State(state): State<AppState>,
    Path(payment_id): Path<Uuid>,
    Json(creds): Json<Credentials>,
) -> Result<String, StatusCode> {
    use crate::session::payment::Event;
    use crate::session::payment::State as PaymentState;

    let session = acquire_session(&state.sessions, payment_id)?;

    // Upgrade session state
    let session_state_guard = session
        .payment_session()
        .ok_or(StatusCode::BAD_REQUEST)?
        .state
        .lock()
        .await;
    let mut watch = session_state_guard.state_finale_notifier.subscribe();
    let fail_url = session_state_guard.req.fail_url.to_string();
    match session_state_guard.state() {
        PaymentState::Init {} => (),
        _ => {
            return {
                tracing::warn!(
                    "Trying to submit payment which is in the {:?} state",
                    session_state_guard.state()
                );
                Err(StatusCode::BAD_REQUEST)
            }
        }
    };
    drop(session_state_guard);

    tokio::spawn(async move {
        let mut session_state_guard = session
            .payment_session()
            .ok_or(StatusCode::BAD_REQUEST)
            .unwrap()
            .state
            .lock()
            .await;
        session_state_guard
            .handle(&Event::Submit {
                bank: state.bank.clone(),
                creds,
            })
            .await;
    });

    // Wait for `ready to capture` state or timeout
    let result = tokio::select! {
        state = watch.changed() => {
            match state {
                Ok(()) => match &*watch.borrow() {
                    PaymentState::Failed {redirect_url, .. }
                    | PaymentState::Closed { redirect_url }
                    | PaymentState::Successed { redirect_url } => {
                        Ok(redirect_url.clone())
                    }
                    _ => unreachable!(),
                }
                Err(e) => {
                    tracing::error!("Failed to get message over channel: {e}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }

        }
        // If there are no actions in 2 minutes, emit Timeout
        _ = tokio::time::sleep(Duration::from_secs(120)) => {
            let session = acquire_session(&state.sessions, payment_id)?;
            let mut session_state_guard = session
                .payment_session()
                .ok_or(StatusCode::BAD_REQUEST)?
                .state
                .lock()
                .await;
            session_state_guard.handle(&Event::Timeout).await;
            Ok(fail_url)
        }
    };
    result
}

#[tracing::instrument(name = "Get card token registration html page", skip_all)]
pub async fn card_token_registration_html_page(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Html<String>, StatusCode> {
    // Try to create submit card for token creation url for client (browser)
    let submit_card_url = match format!(
        "http://{}:{}/card_token/{}",
        state.settings.addr, state.settings.port, id
    )
    .parse()
    {
        Ok(url) => url,
        Err(e) => {
            tracing::error!("Failed to parse string as url: {e}");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Try to return html for card token registration
    match state.sessions.try_acquire_session_by_id(id) {
        Ok(session) => {
            let _ = session
                .card_token_reg_session()
                .ok_or(StatusCode::NOT_FOUND)?;
            match SubmitCardNumberPage::new(submit_card_url).render() {
                Ok(body) => Ok(Html(body)),
                Err(e) => {
                    tracing::error!("Failed to render payment html page: {e}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
        Err(e) => {
            tracing::error!("Failed to get payment html page: {e}");
            Err(StatusCode::NOT_FOUND)
        }
    }
}

/// We return `String` with redirection url.
#[tracing::instrument(name = "Trigger card token registration", skip_all)]
pub async fn trigger_card_token_registration(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    body: String,
) -> Result<String, StatusCode> {
    use crate::session::card_token::Event;
    use crate::session::card_token::State as CardTokenRegState;

    let session = acquire_session(&state.sessions, id)?;

    // Upgrade session state
    let session_state_guard = session
        .card_token_reg_session()
        .ok_or(StatusCode::BAD_REQUEST)?
        .state
        .lock()
        .await;

    // Parse card
    let Ok(card_for_reg) = CardNumber::parse(&body) else {
        tracing::error!("Can't parse card number: {}", body);
        return Ok(session_state_guard.req.fail_url.to_string());
    };

    let mut watch = session_state_guard.state_finale_notifier.subscribe();
    let fail_url = session_state_guard.req.fail_url.to_string();
    match session_state_guard.state() {
        CardTokenRegState::Init {} => (),
        _ => return Err(StatusCode::BAD_REQUEST),
    };
    drop(session_state_guard);

    tokio::spawn(async move {
        let mut session_state_guard = session
            .card_token_reg_session()
            .ok_or(StatusCode::BAD_REQUEST)
            .unwrap()
            .state
            .lock()
            .await;
        session_state_guard
            .handle(&Event::Submit {
                bank: state.bank.clone(),
                card_for_reg,
            })
            .await;
    });

    // Wait for `ready to capture` state or timeout
    let result = tokio::select! {
        state = watch.changed() => {
            match state {
                Ok(()) =>  match &*watch.borrow() {
                    CardTokenRegState::Failed {redirect_url, .. }
                    | CardTokenRegState::Closed { redirect_url }
                    | CardTokenRegState::Successed { redirect_url, .. } => {
                        Ok(redirect_url.clone())
                    }
                    _ => unreachable!(),
                }
                Err(e) => {
                    tracing::error!("Failed to get message over channel: {e}");
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                },
            }

        }
        // If there are no actions in 2 minutes, emit Timeout
        _ = tokio::time::sleep(Duration::from_secs(120)) => {
            let session = acquire_session(&state.sessions, id)?;
            let mut session_state_guard = session
                .card_token_reg_session()
                .ok_or(StatusCode::BAD_REQUEST)?
                .state
                .lock()
                .await;
            session_state_guard.handle(&Event::Timeout).await;
            Ok(fail_url)
        }
    };
    result
}

// ───── Helpers ──────────────────────────────────────────────────────────── //

fn acquire_session(
    sessions: &crate::session::InteractionSessions,
    id: Uuid,
) -> Result<crate::session::Session, StatusCode> {
    match sessions.try_acquire_session_by_id(id) {
        Ok(session) => Ok(session),
        Err(e) => {
            // No such session
            tracing::error!("Session not found: {e}");
            return Err(StatusCode::BAD_REQUEST);
        }
    }
}
