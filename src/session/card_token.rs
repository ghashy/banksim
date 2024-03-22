use std::sync::Arc;
use std::time::Duration;

use banksim_api::notifications::*;
use banksim_api::register_card_token::RegisterCardTokenRequest;
use banksim_api::OperationError;
use banksim_api::OperationStatus;
use statig::awaitable::IntoStateMachineExt;
use statig::state_machine;
use statig::Response;
use time::OffsetDateTime;
use tokio::sync::watch::Sender;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::card_number::CardNumber;
use crate::routes::html_pages_and_triggers::Credentials;

use super::call_webhook;

#[derive(Clone)]
pub struct CardTokenRegSession {
    /// This id for checking session without taking lock on `state` field
    pub id: Uuid,
    pub creation_time: OffsetDateTime,
    pub state: Arc<Mutex<statig::awaitable::StateMachine<Inner>>>,
}

impl CardTokenRegSession {
    pub fn new(
        req: RegisterCardTokenRequest,
        store_credentials: Credentials,
        http_client: reqwest::Client,
        session_watcher_notifier: tokio::sync::oneshot::Sender<()>,
    ) -> CardTokenRegSession {
        let (tx, _) = tokio::sync::watch::channel(State::init());
        let id = Uuid::new_v4();
        let inner = Arc::new(Mutex::new(
            Inner {
                store_credentials,
                req,
                http_client,
                state_finale_notifier: tx,
                card_for_reg: None,
                id,
                session_watcher_notifier: Some(session_watcher_notifier),
            }
            .state_machine(),
        ));
        CardTokenRegSession {
            id,
            creation_time: OffsetDateTime::now_utc(),
            state: inner,
        }
    }
}

impl std::fmt::Debug for CardTokenRegSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "CardTokenRegSession {{ id: {}, creation_time: {}, state: {{redacted}} }}",
            self.id, self.creation_time
        ))
    }
}

#[derive(Debug)]
pub struct Inner {
    pub id: Uuid,
    pub store_credentials: Credentials,
    pub req: RegisterCardTokenRequest,
    pub state_finale_notifier: Sender<State>,
    pub session_watcher_notifier: Option<tokio::sync::oneshot::Sender<()>>,
    http_client: reqwest::Client,
    card_for_reg: Option<CardNumber>,
}

pub enum Event {
    Submit {
        bank: crate::bank::Bank,
        card_for_reg: CardNumber,
    },
    ConfirmRequest {
        bank: crate::bank::Bank,
    },
    Timeout,
    CancelRequest,
}

#[allow(unused_variables)]
#[state_machine(
    initial = "State::init()",
    on_transition = "Self::on_transition",
    state(derive(Debug, Clone))
)]
impl Inner {
    #[state]
    async fn init(&mut self, event: &Event) -> Response<State> {
        match event {
            Event::Submit { bank, card_for_reg } => {
                // Is there any account for provided card?
                let card_for_reg = match bank.find_account(card_for_reg).await {
                    // Authorized
                    Ok(acc) => acc.card(),
                    Err(e) => {
                        // Not authorized
                        tracing::error!("Can't authorize account: {e}");
                        return Response::Transition(State::failed(
                            self.req.fail_url.to_string(),
                            OperationError::NotAuthorizedRequest,
                        ));
                    }
                };

                // Check store account
                // We have only one store account in our virtual bank
                let store_card = match bank.get_store_account().await {
                    Ok(acc) => acc.card(),
                    Err(e) => {
                        tracing::error!("Failed to get store account: {e}");
                        return Response::Transition(State::failed(
                            self.req.fail_url.to_string(),
                            OperationError::NotAuthorizedRequest,
                        ));
                    }
                };
                if !store_card.eq(&self.store_credentials.card_number) {
                    tracing::error!(
                        "Failed to reg token: wrong store account!"
                    );
                    return Response::Transition(State::failed(
                        self.req.fail_url.to_string(),
                        OperationError::NotAuthorizedRequest,
                    ));
                }

                self.card_for_reg = Some(card_for_reg);

                // Webhook future
                let fut = call_webhook(
                    Notification::TokenNotification(
                        TokenNotification::ReadyToConfirm {
                            session_id: self.id.clone(),
                        },
                    ),
                    self.req.notification_url.clone(),
                    self.http_client.clone(),
                );

                // Run with delay
                tokio::spawn(async move {
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    fut.await
                });
                Response::Transition(State::ready_to_confirm())
            }
            Event::Timeout => Response::Transition(State::closed(
                self.req.fail_url.to_string(),
            )),
            _ => Response::Handled,
        }
    }

    #[state]
    async fn ready_to_confirm(&self, event: &Event) -> Response<State> {
        match event {
            Event::ConfirmRequest { bank } => {
                let card_for_reg = self.card_for_reg.as_ref().unwrap();
                let token = match bank.new_card_token(card_for_reg).await {
                    Ok(t) => t,
                    Err(e) => {
                        tracing::error!(
                            "Failed to generate card token: {:?}, error: {e}",
                            self
                        );
                        return Response::Transition(State::failed(
                            self.req.fail_url.to_string(),
                            OperationError::Failed {
                                reason: e.str_reason_for_client(),
                            },
                        ));
                    }
                };
                Response::Transition(State::successed(
                    self.req.success_url.to_string(),
                    token,
                ))
            }
            Event::Timeout | Event::CancelRequest => Response::Transition(
                State::closed(self.req.fail_url.to_string()),
            ),
            _ => Response::Handled,
        }
    }

    #[state]
    fn closed(redirect_url: &String) -> Response<State> {
        Response::Handled
    }

    #[state]
    fn failed(redirect_url: &String, err: &OperationError) -> Response<State> {
        Response::Handled
    }

    #[state]
    fn successed(redirect_url: &String, token: &String) -> Response<State> {
        Response::Handled
    }

    fn on_transition(&mut self, _: &State, target: &State) {
        tracing::info!(
            "Token reg session {} transition to {}",
            self.id,
            target
        );
        let (status, token) = match target {
            State::Successed { token, .. } => {
                (OperationStatus::Success, Some(token.clone()))
            }
            State::Closed { .. } => (OperationStatus::Cancel, None),
            State::Failed { err, .. } => {
                (OperationStatus::Fail(err.clone()), None)
            }
            _ => return,
        };
        self.notify(target.clone());

        // Call webhook
        let id = self.id;
        let url = self.req.notification_url.clone();
        let client = self.http_client.clone();
        tokio::spawn(async move {
            tokio::time::sleep(Duration::from_secs(1)).await;
            call_webhook(
                banksim_api::notifications::Notification::TokenNotification(
                    banksim_api::notifications::TokenNotification::Finished {
                        card_token: token,
                        session_id: id,
                        status,
                    },
                ),
                url,
                client,
            )
            .await
        });
    }

    fn notify(&mut self, state: State) {
        if let Err(e) = self.state_finale_notifier.send(state) {
            tracing::error!(
                "Failed to send notification about finale state: {e}"
            );
        }
        if let Err(e) = self.session_watcher_notifier.take().unwrap().send(()) {
            tracing::error!("Failed to send notification about finale state to task watcher");
        }
    }
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            State::ReadyToConfirm {} => f.write_str("ReadyToConfirm"),
            State::Failed { .. } => f.write_str("Failed"),
            State::Successed { .. } => f.write_str("Successed"),
            State::Closed { .. } => f.write_str("Closed"),
            State::Init {} => f.write_str("Init"),
        }
    }
}
