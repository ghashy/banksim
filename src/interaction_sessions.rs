use std::sync::{Arc, Mutex, MutexGuard};

use acquisim_api::init_payment::InitPaymentRequest;
use acquisim_api::register_card_token::RegisterCardTokenRequest;
use serde::Serialize;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::{domain::card_number::CardNumber, error_chain_fmt, RemovableById};

pub trait IntoSession {
    fn create_session(self, store_card: CardNumber) -> Session;
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

#[derive(Clone)]
pub enum SessionType {
    CardTokenRegistration(RegisterCardTokenRequest),
    Payment(InitPaymentRequest),
}

impl SessionType {
    pub fn register_card_token_req(&self) -> &RegisterCardTokenRequest {
        match self {
            SessionType::CardTokenRegistration(req) => &req,
            SessionType::Payment(_) => panic!("Wrong session type"),
        }
    }

    pub fn payment_req(&self) -> &InitPaymentRequest {
        match self {
            SessionType::CardTokenRegistration(_) => {
                panic!("Foudn CardTokenRegistration() in payment")
            }
            SessionType::Payment(req) => &req,
        }
    }
}

/// Represents continuous processes with beginning and end,
/// can be initialized, become active, be expired or finished.
#[derive(Clone)]
pub struct Session {
    pub session_type: SessionType,
    pub store_card: CardNumber,
    pub creation_time: OffsetDateTime,
    id: Uuid,
}

impl Session {
    pub fn id(&self) -> Uuid {
        self.id
    }

    fn new_payment_session(
        req: InitPaymentRequest,
        store_card: CardNumber,
    ) -> Self {
        Session {
            session_type: SessionType::Payment(req),
            store_card,
            creation_time: OffsetDateTime::now_utc(),
            id: Uuid::new_v4(),
        }
    }

    fn new_card_token_registration_session(
        req: RegisterCardTokenRequest,
        store_card: CardNumber,
    ) -> Self {
        Session {
            session_type: SessionType::CardTokenRegistration(req),
            store_card,
            creation_time: OffsetDateTime::now_utc(),
            id: Uuid::new_v4(),
        }
    }

    async fn notify<T: Serialize + ?Sized + std::fmt::Debug>(
        &self,
        client: &reqwest::Client,
        notification: &T,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let url = match self.session_type {
            SessionType::CardTokenRegistration(ref req) => {
                &req.notification_url
            }
            SessionType::Payment(ref req) => &req.notification_url,
        };
        client.post(url.clone()).json(notification).send().await
    }
}

#[derive(Clone)]
pub struct InteractionSessions {
    list: Arc<Mutex<Vec<Session>>>,
    http_client: reqwest::Client,
}

impl InteractionSessions {
    pub fn new() -> Self {
        InteractionSessions {
            list: Arc::new(Mutex::new(Vec::new())),
            http_client: reqwest::Client::new(),
        }
    }

    pub fn insert(&self, entity: Session) -> Result<(), SessionError> {
        Ok(self.lock()?.push(entity))
    }

    pub fn try_acquire_session_by_id(
        &self,
        id: Uuid,
    ) -> Result<Session, SessionError> {
        let lock = self.lock()?;
        if let Some(entity) = lock.iter().find(|p| p.id().eq(&id)) {
            Ok(entity.clone())
        } else {
            Err(SessionError::NoEntityError(id))
        }
    }

    pub async fn notify_and_remove<T: Serialize + ?Sized + std::fmt::Debug>(
        &self,
        id: Uuid,
        notification: &T,
    ) {
        match self.try_acquire_session_by_id(id) {
            Ok(entity) => {
                if let Err(e) =
                    entity.notify(&self.http_client, notification).await
                {
                    tracing::error!(
                        "Failed to notify with {notification:?}, error: {e}"
                    )
                }
            }
            Err(e) => {
                tracing::error!(
                    "Failed to acquire entity with id: {id}, error: {e}"
                );
            }
        }
        if let Err(e) = self.remove_session_by_id(id) {
            tracing::error!(
                "Failed to remove entity with id {id} from list: {e}"
            );
        }
    }

    fn remove_session_by_id(&self, id: Uuid) -> Result<(), SessionError> {
        let mut lock = self.lock()?;
        if let Some(pos) = lock.iter().position(|p| p.id().eq(&id)) {
            let _ = lock.swap_remove(pos);
        }
        Ok(())
    }

    fn lock(&self) -> Result<MutexGuard<Vec<Session>>, SessionError> {
        self.list
            .lock()
            .map_err(|e| SessionError::MutexError(e.to_string()))
    }
}

impl RemovableById for InteractionSessions {
    fn remove(&self, id: Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.remove_session_by_id(id).map_err(|e| e.into())
    }
}

impl IntoSession for InitPaymentRequest {
    fn create_session(self, store_card: CardNumber) -> Session {
        Session::new_payment_session(self, store_card)
    }

    fn page_endpoint() -> &'static str {
        "payment_page"
    }
}

impl IntoSession for RegisterCardTokenRequest {
    fn create_session(self, store_card: CardNumber) -> Session {
        Session::new_card_token_registration_session(self, store_card)
    }

    fn page_endpoint() -> &'static str {
        "register_card_token_page"
    }
}
