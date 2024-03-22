use std::sync::{Arc, Mutex, MutexGuard};
use std::time::Duration;

use banksim_api::init_payment::InitPaymentRequest;
use banksim_api::register_card_token::RegisterCardTokenRequest;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::routes::html_pages_and_triggers::Credentials;
use crate::{error_chain_fmt, RemovableById};

use self::card_token::CardTokenRegSession;
use self::payment::PaymentSession;

pub mod card_token;
pub mod payment;

pub trait IntoSession {
    fn create_session(
        self,
        store_credentials: Credentials,
        http_client: reqwest::Client,
        session_watcher_notifier: tokio::sync::oneshot::Sender<()>,
    ) -> Session;
    fn page_endpoint() -> &'static str;
}

// ───── Error Type ───────────────────────────────────────────────────────── //

#[derive(thiserror::Error)]
pub enum SessionError {
    #[error("Can't take a mutex lock: {0}")]
    MutexError(String),
    #[error("No entity with provided id: {0}")]
    NoEntityError(Uuid),
}

impl std::fmt::Debug for SessionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}

// ───── Session ──────────────────────────────────────────────────────────── //

/// Represents continuous processes with beginning and end,
/// can be initialized, become active, be expired or finished.
#[derive(Debug, Clone)]
pub enum Session {
    PaymentSession(PaymentSession),
    CardTokenRegSession(CardTokenRegSession),
}

impl Session {
    pub fn id(&self) -> Uuid {
        match self {
            Session::PaymentSession(s) => s.id,
            Session::CardTokenRegSession(s) => s.id,
        }
    }
    pub fn creation_time(&self) -> OffsetDateTime {
        match self {
            Session::PaymentSession(s) => s.creation_time,
            Session::CardTokenRegSession(s) => s.creation_time,
        }
    }

    fn new_payment_session(
        req: InitPaymentRequest,
        store_credentials: Credentials,
        http_client: reqwest::Client,
        session_watcher_notifier: tokio::sync::oneshot::Sender<()>,
    ) -> Self {
        Session::PaymentSession(PaymentSession::new(
            req,
            store_credentials,
            http_client,
            session_watcher_notifier,
        ))
    }
    pub fn payment_session(&self) -> Option<&PaymentSession> {
        match self {
            Session::PaymentSession(s) => Some(s),
            Session::CardTokenRegSession(_) => None,
        }
    }

    fn new_card_token_registration_session(
        req: RegisterCardTokenRequest,
        store_credentials: Credentials,
        http_client: reqwest::Client,
        session_watcher_notifier: tokio::sync::oneshot::Sender<()>,
    ) -> Self {
        Session::CardTokenRegSession(CardTokenRegSession::new(
            req,
            store_credentials,
            http_client,
            session_watcher_notifier,
        ))
    }
    pub fn card_token_reg_session(&self) -> Option<&CardTokenRegSession> {
        match self {
            Session::PaymentSession(_) => None,
            Session::CardTokenRegSession(s) => Some(s),
        }
    }
}

#[derive(Clone)]
pub struct InteractionSessions {
    list: Arc<Mutex<Vec<Session>>>,
}

impl InteractionSessions {
    pub fn new() -> Self {
        InteractionSessions {
            list: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn insert(&mut self, entity: Session) -> Result<(), SessionError> {
        println!("Inserting entity: {:?}", &entity);
        Ok(self.lock()?.push(entity))
    }

    pub fn try_acquire_session_by_id(
        &self,
        id: Uuid,
    ) -> Result<Session, SessionError> {
        if let Some(entity) = self.lock()?.iter().find(|p| p.id().eq(&id)) {
            Ok(entity.clone())
        } else {
            Err(SessionError::NoEntityError(id))
        }
    }

    pub fn remove_session_by_id(
        &mut self,
        id: Uuid,
    ) -> Result<(), SessionError> {
        let mut guard = self.lock()?;
        if let Some(pos) = guard.iter().position(|p| p.id().eq(&id)) {
            let _ = guard.swap_remove(pos);
            Ok(())
        } else {
            Err(SessionError::NoEntityError(id))
        }
    }

    fn lock(&self) -> Result<MutexGuard<Vec<Session>>, SessionError> {
        self.list.lock().map_err(|e| SessionError::MutexError(e.to_string()))
    }
}

impl RemovableById for InteractionSessions {
    fn remove(&mut self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.remove_session_by_id(id).map_err(|e| e.into())
    }

    fn exists(&self, id: Uuid) -> Result<bool, Box<dyn std::error::Error>> {
        self.lock()
            .map(|list| list.iter().find(|s| s.id().eq(&id)).is_some())
            .map_err(|e| e.into())
    }
}

impl IntoSession for InitPaymentRequest {
    fn create_session(
        self,
        store_credentials: Credentials,
        http_client: reqwest::Client,
        session_watcher_notifier: tokio::sync::oneshot::Sender<()>,
    ) -> Session {
        Session::new_payment_session(
            self,
            store_credentials,
            http_client,
            session_watcher_notifier,
        )
    }

    fn page_endpoint() -> &'static str {
        "payment_page"
    }
}

impl IntoSession for RegisterCardTokenRequest {
    fn create_session(
        self,
        store_credentials: Credentials,
        http_client: reqwest::Client,
        session_watcher_notifier: tokio::sync::oneshot::Sender<()>,
    ) -> Session {
        Session::new_card_token_registration_session(
            self,
            store_credentials,
            http_client,
            session_watcher_notifier,
        )
    }

    fn page_endpoint() -> &'static str {
        "register_card_token_page"
    }
}

async fn call_webhook<T: Serialize + 'static>(
    notification: T,
    notification_url: url::Url,
    http_client: reqwest::Client,
) {
    // Call webhook, 3 attempts
    for _ in 0..3 {
        match http_client
            .post(notification_url.clone())
            .json(&notification)
            .send()
            .await
        {
            Ok(r) => {
                if r.status().as_u16() == 200 {
                    break;
                } else {
                    tracing::warn!(
                        "Got {} status from webhook call",
                        r.status().as_u16()
                    );
                    break;
                }
            }
            Err(e) => tracing::warn!("Failed to call webhook: {e}"),
        }
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
