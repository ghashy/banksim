use std::sync::Arc;
use std::time::Duration;

use banksim_api::init_payment::InitPaymentRequest;
use banksim_api::notifications::*;
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
pub struct PaymentSession {
    /// This id for checking payment session without taking lock on `state` field
    pub id: Uuid,
    pub creation_time: OffsetDateTime,
    pub state: Arc<Mutex<statig::awaitable::StateMachine<Inner>>>,
}

impl PaymentSession {
    pub fn new(
        req: InitPaymentRequest,
        store_credentials: Credentials,
        http_client: reqwest::Client,
        session_watcher_notifier: tokio::sync::oneshot::Sender<()>,
    ) -> PaymentSession {
        let (tx, _) = tokio::sync::watch::channel(State::init());
        let id = Uuid::new_v4();
        let inner = Arc::new(Mutex::new(
            Inner {
                store_credentials,
                req,
                http_client,
                state_finale_notifier: tx,
                payer_card: None,
                session_watcher_notifier: Some(session_watcher_notifier),
                id,
            }
            .state_machine(),
        ));
        PaymentSession {
            id,
            creation_time: OffsetDateTime::now_utc(),
            state: inner,
        }
    }
}

impl std::fmt::Debug for PaymentSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "PaymentSession {{ id: {}, creation_time: {}, state: {{redacted}} }}",
            self.id, self.creation_time
        ))
    }
}

#[derive(Debug)]
pub struct Inner {
    pub id: Uuid,
    pub store_credentials: Credentials,
    pub req: InitPaymentRequest,
    pub state_finale_notifier: Sender<State>,
    pub session_watcher_notifier: Option<tokio::sync::oneshot::Sender<()>>,
    http_client: reqwest::Client,
    payer_card: Option<CardNumber>,
}

pub enum Event {
    Submit {
        bank: crate::bank::Bank,
        creds: crate::routes::html_pages_and_triggers::Credentials,
    },
    Timeout,
    ConfirmRequest,
    CaptureRequest {
        bank: crate::bank::Bank,
    },
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
            Event::Submit { bank, creds } => {
                // Authorize payer's card and password
                let payer_card = match bank
                    .authorize_account(&creds.card_number, &creds.password)
                    .await
                {
                    // Authorized
                    Ok(acc) => acc.card(),
                    Err(e) => {
                        // Not authorized
                        tracing::error!("Can't authorize account: {e:?}");
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
                        "Failed to perform payment: wrong store account!"
                    );
                    return Response::Transition(State::failed(
                        self.req.fail_url.to_string(),
                        OperationError::NotAuthorizedRequest,
                    ));
                }

                self.payer_card = Some(payer_card);

                // Webhook future
                let fut = call_webhook(
                    Notification::PaymentNotification(
                        PaymentNotification::ReadyToConfirm {
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
            Event::ConfirmRequest => {
                let fut = call_webhook(
                    Notification::PaymentNotification(
                        PaymentNotification::ReadyToCapture {
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
                Response::Transition(State::ready_to_capture())
            }
            Event::Timeout | Event::CancelRequest => Response::Transition(
                State::closed(self.req.fail_url.to_string()),
            ),
            _ => Response::Handled,
        }
    }

    #[state]
    async fn ready_to_capture(&self, event: &Event) -> Response<State> {
        match event {
            Event::CaptureRequest { bank } => {
                let payer_card = self.payer_card.as_ref().unwrap();
                // Perform transaction
                let result = if self.req.beneficiaries.is_empty() {
                    bank.new_transaction(
                        payer_card,
                        &self.store_credentials.card_number,
                        self.req.amount,
                    )
                    .await
                } else {
                    bank.new_split_transaction(
                        &payer_card,
                        self.req.amount,
                        &self.req.beneficiaries,
                    )
                    .await
                };
                match result {
                    Ok(()) => Response::Transition(State::successed(
                        self.req.success_url.to_string(),
                    )),
                    Err(e) => {
                        tracing::error!("Transaction failed: {e}");
                        Response::Transition(State::failed(
                            self.req.fail_url.to_string(),
                            OperationError::Failed {
                                reason: e.str_reason_for_client(),
                            },
                        ))
                    }
                }
            }
            Event::Timeout | Event::CancelRequest => Response::Transition(
                State::closed(self.req.fail_url.to_string()),
            ),
            _ => Response::Handled,
        }
    }

    #[state]
    fn closed(&mut self, redirect_url: &String) -> Response<State> {
        Response::Handled
    }

    #[state]
    fn failed(redirect_url: &String, err: &OperationError) -> Response<State> {
        Response::Handled
    }

    #[state]
    fn successed(redirect_url: &String) -> Response<State> {
        Response::Handled
    }

    fn on_transition(&mut self, _: &State, target: &State) {
        tracing::info!(
            "Payment session {} transition to {:?}",
            self.id,
            target
        );
        let status = match target {
            State::Successed { .. } => OperationStatus::Success,
            State::Closed { .. } => OperationStatus::Cancel,
            State::Failed { err, .. } => OperationStatus::Fail(err.clone()),
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
                Notification::PaymentNotification(
                    PaymentNotification::PaymentFinished {
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
